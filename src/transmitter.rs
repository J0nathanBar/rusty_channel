use std::sync::Arc;

use bytes::Bytes;
use tokio::{net::UdpSocket, sync::mpsc};

pub struct UdpTransmitter {
    socket: Arc<UdpSocket>,
    data_source: mpsc::Receiver<Bytes>,
    dest_addr: String,
}

impl UdpTransmitter {
    pub async fn new(
        src_addr: String,
        data_source: mpsc::Receiver<Bytes>,
        dest_addr: String,
    ) -> Result<UdpTransmitter, Box<dyn std::error::Error>> {
        let socket = Arc::new(UdpSocket::bind(src_addr).await?);
        Ok(UdpTransmitter {
            socket,
            data_source,
            dest_addr: dest_addr,
        })
    }
    pub async fn run(&mut self) {
        loop {
            let data = self.data_source.recv().await;
            if let Some(data) = data {
                tokio::spawn(UdpTransmitter::send_data(
                    self.socket.clone(),
                    data,
                    self.dest_addr.clone(),
                ));
            } else {
                break;
            }
        }
    }
    async fn send_data(sock: Arc<UdpSocket>, data: Bytes, dest: String) {
        let send_res = sock.send_to(&data, dest).await;
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
