//! # Tranmitter
//!
//! A module for controlling the transmission of data via UDP.
use std::sync::Arc;
use tokio::{net::UdpSocket, sync::mpsc};

/// A UDP transmitter
///
/// Contains a `tokio::UdpSocket`
/// for data transmission
///
/// and a `tokio::mpsc::Reciever` to get data to send
pub struct UdpTransmitter {
    socket: Arc<UdpSocket>,
    data_source_recv: mpsc::Receiver<Vec<u8>>,
}
const CHANNEL_BUFFER: usize = 1024; //TODO figure out the actual buffer size
impl UdpTransmitter {
    /// This function will create a new `UdpTransmitter` along with a `Sender` for communications
    /// with given ip addresses for src and transmission using `String`
    /// and a `tokio::mpsc::Reciever`
    ///
    ///  # Notes
    ///
    /// The caller is responsible for the validity of the adresses
    /// in case of an invalid address the function will return an `Error`
    ///
    /// # Example
    /// ```no_run
    ///
    /// use channel_networking::UdpTransmitter;
    /// use tokio::sync::mpsc;
    ///
    /// async fn do_something()
    /// {
    ///     let dest_addr = String::from("127.0.0.1:6943");
    ///     let (mut tx,data_tx) =
    ///     UdpTransmitter::new(String::from("127.0.0.1:6942"), dest_addr.clone()).
    ///     await.
    ///     unwrap();
    ///     //use UdpTransmitter...
    /// }
    pub async fn new(
        src_addr: String,
        dest_addr: String,
    ) -> Result<(UdpTransmitter, mpsc::Sender<Vec<u8>>), Box<dyn std::error::Error>> {
        let (data_send, data_source_recv) = mpsc::channel(CHANNEL_BUFFER);
        let socket = Arc::new(UdpSocket::bind(src_addr).await?);
        socket.connect(dest_addr).await?;
        Ok((
            UdpTransmitter {
                socket,
                data_source_recv,
            },
            data_send,
        ))
    }

    /// This function is responsible for the continuous run of the `UdpTransmitter`
    ///
    /// `Run` will loop until the channel is closed.
    ///
    /// The function awaits data from the loop and transfers it to the `UdpSocket`
    /// # Example
    /// ```no_run
    ///
    /// use channel_networking::UdpTransmitter;
    /// use tokio::sync::mpsc;
    ///
    /// async fn do_something(){
    ///     let dest_addr = String::from("127.0.0.1:6943");
    ///   
    ///     let (mut tx,mut data_tx) =
    ///     UdpTransmitter::new(String::from("127.0.0.1:6942"), dest_addr.clone()).
    ///     await.
    ///     unwrap();
    ///     tx.run().await;
    /// }
    pub async fn run(&mut self) {
        loop {
            let data = self.data_source_recv.recv().await;
            if let Some(data) = data {
                println!("prepping to send data");
                tokio::spawn(UdpTransmitter::send_data(self.socket.clone(), data));
            } else {
                break;
            }
        }
    }
    async fn send_data(sock: Arc<UdpSocket>, data: Vec<u8>) {
        let send_res = sock.send(&data).await;
        if let Ok(bytes_sent) = send_res {
            println!("Sent {} bytes successfully", bytes_sent);
        } else {
            eprintln!("Error sending data: {}", send_res.unwrap_err());
        }
    }
}

impl Drop for UdpTransmitter {
    fn drop(&mut self) {
        println!("goodbye!");
    }
}
