use serde::{Deserialize, Serialize};
use std::{sync::atomic::AtomicUsize, time};
type IdType = usize;

#[derive(Serialize, Deserialize)]
pub struct InfoPakcet {
    data_size: usize,
    id: IdType, //TODO generate a uuid
    destination: (),
}
impl InfoPakcet {
     fn new(destination: (), data_size: usize) -> InfoPakcet {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        InfoPakcet {
            data_size,
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            destination,
        }
    }
    pub fn get_id(&self) -> IdType {
        self.id
    }
}
#[derive(Serialize, Deserialize)]
pub struct DataPacket {
    data: Vec<u8>, //Data is always after fec
    id: IdType,
}
impl DataPacket {
     fn new(data: Vec<u8>, id: IdType) -> DataPacket {
        DataPacket { data, id }
    }
}
#[derive(Serialize, Deserialize)]
pub struct KeepAlivePacket {}

#[derive(Serialize, Deserialize)]
pub enum Packet {
    Info(InfoPakcet),
    Data(DataPacket),
    KeepAlive(KeepAlivePacket),
}

impl Packet {
    pub fn new_info_packet(data_size:usize)->Packet {
        return Packet::Info(InfoPakcet::new((), data_size));
    }
    pub fn new_data_packet(data:Vec<u8>,id:IdType)->Packet {
        return Packet::Data(DataPacket::new(data, id));
    }
    pub fn new_keep_alive_packet()->Packet {
        return Packet::KeepAlive(KeepAlivePacket {  });
    }
}