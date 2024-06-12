use bytes::Bytes;
use channel_networking::UdpTransmitter;

#[tokio::main]
async fn main() {
    let src = String::from("127.0.0.1:26969");
    let dst = String::from("127.0.0.1:26970");
    let (mut tx, chan) = UdpTransmitter::new(src, dst).await.unwrap();
    let handler = tokio::spawn(async move { tx.run().await });
    let data = Bytes::from("hello bro");
    chan.send(data).await.unwrap();
    handler.await.unwrap();
}
