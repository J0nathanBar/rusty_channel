use tokio::{net::UdpSocket, sync::mpsc};

const BUFFER_SIZE: usize = 1024 * 4; //TODO: buffer size should be dynamic based on info packet
pub struct UdpReciever {
    socket: UdpSocket,
    data_destination: mpsc::Sender<(Vec<u8>, usize)>,
}

impl UdpReciever {
    pub async fn new(
        addr: String,
        data_destination: mpsc::Sender<(Vec<u8>, usize)>,
    ) -> Result<UdpReciever, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind(addr).await?;
        Ok(UdpReciever {
            socket,
            data_destination,
        })
    }

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
        let (tx_chan, mut rx_chan) = mpsc::channel(1024);
        let mut rx = UdpReciever::new(String::from(dest), tx_chan).await.unwrap();
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
