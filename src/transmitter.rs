//! # Tranmitter
//!
//! A module for controlling the transmission of data via UDP.
use bytes::Bytes;
use std::sync::Arc;
use tokio::{net::UdpSocket, sync::mpsc};

/// A UDP transmitter
///
/// Contains a `tokio::UdpSocket`
/// for data transmission
///
/// and a channel reciever to get data to send
pub struct UdpTransmitter {
    socket: Arc<UdpSocket>,
    data_source: mpsc::Receiver<Bytes>,
}

impl UdpTransmitter {
    /// This function will create a new Transmitter
    /// with given ip addresses for src and transmission and a reciever channel
    ///
    ///  # Notes
    ///
    /// The caller is responsible for the validity of the adresses
    /// in case of an invalid address the function will return an `Error`
    ///
    /// # Example
    /// ```no_run
    ///
    /// use rusty_channel::transmitter::UdpTransmitter;
    /// use tokio::sync::mpsc;
    ///
    ///
    /// let dest_addr = String::from("127.0.0.1:6943");
    /// let (data_tx, data_rx) = mpsc::channel(1);
    /// let mut tx =
    /// UdpTransmitter::new(String::from("127.0.0.1:6942"), data_rx, dest_addr.clone()).
    /// await.
    /// unwrap();
    /// //use UdpTransmitter...
    pub async fn new(
        src_addr: String,
        data_source: mpsc::Receiver<Bytes>,
        dest_addr: String,
    ) -> Result<UdpTransmitter, Box<dyn std::error::Error>> {
        let socket = Arc::new(UdpSocket::bind(src_addr).await?);
        socket.connect(dest_addr).await?;
        Ok(UdpTransmitter {
            socket,
            data_source,
        })
    }

    /// This function is responsible for the continuous run of the transmitter
    ///
    /// `Run` will loop until the channel is closed.
    ///
    /// The function awaits data from the loop and transfers it to the socket
    /// # Example
    /// ```no_run
    ///
    /// use rusty_channel::transmitter::UdpTransmitter;
    /// use tokio::sync::mpsc;
    ///
    ///
    /// let dest_addr = String::from("127.0.0.1:6943");
    /// let (data_tx, data_rx) = mpsc::channel(1);
    /// let mut tx =
    /// UdpTransmitter::new(String::from("127.0.0.1:6942"), data_rx, dest_addr.clone()).
    /// await.
    /// unwrap();
    /// tx.run().await;
    pub async fn run(&mut self) {
        loop {
            let data = self.data_source.recv().await;
            if let Some(data) = data {
                tokio::spawn(UdpTransmitter::send_data(self.socket.clone(), data));
            } else {
                break;
            }
        }
    }
    async fn send_data(sock: Arc<UdpSocket>, data: Bytes) {
        let send_res = sock.send(&data).await;
        if let Ok(bytes_sent) = send_res {
            println!("Sent {} bytes successfully", bytes_sent);
        } else {
            eprintln!("Error sending data: {}", send_res.unwrap_err());
        }
    }
}

#[cfg(test)]
mod transmitter_tests {
    use super::*;
    #[tokio::test]
    async fn test_transmitter() {
        let dest_addr = String::from("127.0.0.1:6943");
        let (data_tx, data_rx) = mpsc::channel(1);
        let mut tx =
            UdpTransmitter::new(String::from("127.0.0.1:6942"), data_rx, dest_addr.clone())
                .await
                .unwrap();
        let sock = UdpSocket::bind(dest_addr).await.unwrap();
        tokio::spawn(async move { tx.run().await });
        let data_to_send = Bytes::from(vec![0u8; 200]);
        let len = data_to_send.len();
        data_tx.send(data_to_send).await.unwrap();
        let mut buff = vec![0u8; 1024];
        let mnt = sock.recv(&mut buff).await;
        assert_eq!(len, mnt.unwrap());
    }
}
