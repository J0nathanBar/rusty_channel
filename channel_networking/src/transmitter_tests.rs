#[cfg(test)]
mod transmitter_tests {
    use tokio::net::UdpSocket;

    use crate::UdpTransmitter;

    #[tokio::test]
    async fn test_transmitter() {
        // let dest_addr = String::from("127.0.0.1:6943");
        // let (mut tx, data_tx) =
        //     UdpTransmitter::new(String::from("127.0.0.1:6942"), dest_addr.clone())
        //         .await
        //         .unwrap();
        // let sock = UdpSocket::bind(dest_addr).await.unwrap();
        // tokio::spawn(async move { tx.run().await });
        // let data_to_send = vec![0u8; 200];
        // let len = data_to_send.len();
        // data_tx.send(data_to_send).await.unwrap();
        // let mut buff = vec![0u8; 1024];
        // let mnt = sock.recv(&mut buff).await;
        // assert_eq!(len, mnt.unwrap());
    }
}
