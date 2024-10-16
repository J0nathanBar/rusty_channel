#[cfg(test)]
mod reciever_tests {
    use tokio::net::UdpSocket;

    use crate::UdpReciever;

    #[tokio::test]
    async fn test_reciever() {
        // const SEND_LEN: usize = 69;
        // let dest = "127.0.0.1:6948";
        // let (mut rx, mut rx_chan) = UdpReciever::new(String::from(dest)).await.unwrap();
        // let sock = UdpSocket::bind("127.0.0.1:6969").await.unwrap();
        // sock.connect(String::from(dest)).await.unwrap();
        // let data = vec![69u8; SEND_LEN];
        // let cpy = data.clone();
        // let incoming = tokio::spawn(async move { rx_chan.recv().await });
        // tokio::spawn(async move { rx.run().await });
        // sock.send(&data).await.unwrap();
        // let res = incoming.await.unwrap().unwrap();
        // assert_eq!(&res.0[..res.1], cpy);
    }
}
