use std::sync::Arc;
use tokio::{net::UdpSocket, sync::mpsc};

const BUFFER_SIZE: usize = 1024 * 4; //TODO: buffer size should be dynamic based on info packet
pub struct UdpReciever {
    socket: UdpSocket,
    data_destination: Arc<mpsc::Sender<Vec<u8>>>,
}

impl UdpReciever {
    pub async fn new(
        addr: String,
        data_destination: mpsc::Sender<Vec<u8>>,
    ) -> Result<UdpReciever, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind(addr).await?;
        let data_destination = Arc::new(data_destination);
        Ok(UdpReciever {
            socket,
            data_destination,
        })
    }

    pub async fn run(&mut self) {
        loop {
            let mut buf = vec![0u8; BUFFER_SIZE];
            let recieve_result = self.socket.recv(&mut buf).await;
            if let Ok(bytes_recieved) = recieve_result {
                let dest = self.data_destination.clone();
                tokio::spawn(async move { UdpReciever::handle_data(dest, buf) });
                println!("Recieved {} bytes successfully", bytes_recieved);
            } else {
                eprintln!("Error: {:?}", recieve_result);
            }
        }
    }
    async fn handle_data(data_dest: Arc<mpsc::Sender<Vec<u8>>>, buf: Vec<u8>) {
        let send_res = data_dest.send(buf).await;
        if let Err(e) = send_res {
            eprintln!("Error handling data: {:?}", e);
        }
    }
}
