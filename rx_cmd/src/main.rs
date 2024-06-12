use channel_networking::UdpReciever;

#[tokio::main]
async fn main() {
    let addr = String::from("127.0.0.1:26970");
    let (mut rx, mut chan) = UdpReciever::new(addr).await.unwrap();
    tokio::spawn(async move { rx.run().await });
    let data = chan.recv().await.unwrap();
    println!(
        "got: {} bytes which make up for this string: {}",
        data.1,
        String::from_utf8(data.0).unwrap()
    );
}
