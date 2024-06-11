//! Reciever
//!
//! A module responsible the reception of data via UDP
use tokio::{net::UdpSocket, sync::mpsc};

const BUFFER_SIZE: usize = 1024 * 4; //TODO: buffer size should be dynamic based on info packet
/// A UDP reciever
///
/// Contains a `tokio::socket`
/// for data reception
///
/// and a `tokio::mpsc::Sender` for continual of data manipulation
pub struct UdpReciever {
    socket: UdpSocket,
    data_destination: mpsc::Sender<(Vec<u8>, usize)>,
}
impl UdpReciever {
    /// This function will create a new `UdpReciever` and an `mpsc::Reciever` for communications
    ///
    /// It takes an address to `bind` to
    /// for data forwarding
    ///
    /// # Notes
    ///
    /// The caller is responsible for the validity of the adresses
    /// in case of an invalid address the function will return an `Error`
    ///
    /// # Example
    /// ```no_run
    ///
    /// use channel_networking::UdpReciever;
    /// use tokio::sync::mpsc;
    ///
    /// async fn do_something()
    /// {
    ///     let dest = "127.0.0.1:6948";
    ///     let (mut rx,rx_chan) = UdpReciever::new(String::from(dest)).await.unwrap();
    ///     //use `UdpReciever`...
    /// }
    pub async fn new(
        addr: String,
    ) -> Result<(UdpReciever, mpsc::Receiver<(Vec<u8>, usize)>), Box<dyn std::error::Error>> {
        let (data_destination, data_handle_channel) = mpsc::channel(BUFFER_SIZE);
        let socket = UdpSocket::bind(addr).await?;
        Ok((
            UdpReciever {
                socket,
                data_destination,
            },
            data_handle_channel,
        ))
    }
    /// This function is responsible for the continuous run of the `UdpReciever`
    ///
    /// `Run` will loop forever.
    ///
    /// The function awaits data from the loop and transfers it to the socket
    /// # Example
    /// ```no_run
    ///
    /// use channel_networking::UdpReciever;
    /// use tokio::sync::mpsc;
    ///
    ///
    /// async fn do_something()
    /// {
    ///     let dest = "127.0.0.1:6948";
    ///     let (mut rx,mut rx_chan) = UdpReciever::new(String::from(dest)).await.unwrap();
    ///     let incoming = tokio::spawn(async move { rx_chan.recv().await });
    ///     tokio::spawn(async move { rx.run().await });
    ///     let res = incoming.await.unwrap().unwrap();
    ///     let data = &res.0[..res.1];
    ///     //Use data as necessary
    /// }
    pub async fn run(&mut self) {
        loop {
            let mut buf = vec![0u8; BUFFER_SIZE];
            let recieve_result = self.socket.recv(&mut buf).await;
            if let Ok(bytes_recieved) = recieve_result {
                let dest = self.data_destination.clone();
                tokio::spawn(
                    async move { UdpReciever::handle_data(dest, buf, bytes_recieved).await },
                );
                println!("Recieved {} bytes successfully", bytes_recieved);
            } else {
                eprintln!("Error: {:?}", recieve_result);
            }
        }
    }
    async fn handle_data(
        data_dest: mpsc::Sender<(Vec<u8>, usize)>,
        buf: Vec<u8>,
        bytes_recieved: usize,
    ) {
        let send_res = data_dest.send((buf, bytes_recieved)).await;
        if let Err(e) = send_res {
            eprintln!("Error handling data: {:?}", e);
        }
    }
}

#[cfg(test)]
mod reciever_tests {
    use super::*;
    #[tokio::test]
    async fn test_reciever() {
        const SEND_LEN: usize = 69;
        let dest = "127.0.0.1:6948";
        let (mut rx, mut rx_chan) = UdpReciever::new(String::from(dest)).await.unwrap();
        let sock = UdpSocket::bind("127.0.0.1:6969").await.unwrap();
        sock.connect(String::from(dest)).await.unwrap();
        let data = vec![69u8; SEND_LEN];
        let cpy = data.clone();
        let incoming = tokio::spawn(async move { rx_chan.recv().await });
        tokio::spawn(async move { rx.run().await });
        sock.send(&data).await.unwrap();
        let res = incoming.await.unwrap().unwrap();
        assert_eq!(&res.0[..res.1], cpy);
    }
}
