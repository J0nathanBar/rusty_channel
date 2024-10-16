use channel_networking::{packets::Packet, UdpReciever};

#[tokio::main]
async fn main() {
    let addr = String::from("127.0.0.1:26970");
    let (mut rx, mut chan) = UdpReciever::new(addr).await.unwrap();
    tokio::spawn(async move { rx.run().await });
    let (data,_) = chan.recv().await.unwrap();
    let packet:Packet = bincode::deserialize(&data[..]).unwrap();
    handle_packet(packet);
    let (data,_) = chan.recv().await.unwrap();
    let packet:Packet = bincode::deserialize(&data[..]).unwrap();
    handle_packet(packet);
}

fn handle_packet(packet:Packet) {
    match packet {
        Packet::Info(info_pakcet) => println!("info! incoming size: {} bytes",info_pakcet.get_data_size()),
        Packet::Data(data_packet) => println!("data! {}",String::from_utf8(data_packet.consume_payload()).unwrap()),
        Packet::KeepAlive(_) => todo!(),
    }
}
