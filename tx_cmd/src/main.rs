
use channel_networking::{packets::{InfoPakcet, Packet}, UdpTransmitter};

#[tokio::main]
async fn main() {
    let data = String::from("hello bro").into_bytes();
    let length = data.len();
    let src = String::from("127.0.0.1:26969");
    let dst = String::from("127.0.0.1:26970");
    let (mut tx, chan) = UdpTransmitter::new(src, dst).await.unwrap();
    let handler = tokio::spawn(async move { tx.run().await });
    chan.send(data).await.unwrap();
    drop(chan);
    handler.await.unwrap();
}


