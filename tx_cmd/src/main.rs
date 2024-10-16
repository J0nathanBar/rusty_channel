
use channel_networking::{packets::Packet, UdpTransmitter};

#[tokio::main]
async fn main() {
    
    let src = String::from("127.0.0.1:26969");
    let dst = String::from("127.0.0.1:26970");
    let (mut tx, chan) = UdpTransmitter::new(src, dst).await.unwrap();
    let handler = tokio::spawn(async move { tx.run().await });

    let data = String::from("hello bro").into_bytes();
    let length = data.len();

    let info = Packet::new_info_packet(length);
    chan.send(bincode::serialize(&info).unwrap()).await.unwrap();
    let data_packet = Packet::new_data_packet(data,0);
    chan.send(bincode::serialize(&data_packet).unwrap()).await.unwrap();
    drop(chan);
    handler.await.unwrap();
}


