use std::error::Error;

use tokio::sync::mpsc;

use crate::fec_config::FecConfig;

type SenderChannel = mpsc::Sender<(Vec<u8>, u32)>; //data and fec id
pub struct FecEncoder {
    config: FecConfig,
    out_channel: SenderChannel,
}

impl FecEncoder {
    pub fn new(config: FecConfig, chan: SenderChannel) -> FecEncoder {
        FecEncoder {
            config,
            out_channel: chan,
        }
    }
    pub fn encode(self, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let mut encoder =
            raptor_code::SourceBlockEncoder::new(&data, self.config.source_symbols_num);
        let packet_number = encoder.nb_source_symbols() + self.config.nb_repair;
        for fec_id in 0..packet_number {
            let symbol = encoder.fountain(fec_id);
            self.out_channel.blocking_send((symbol, fec_id))?;
        }
        drop(self.out_channel);
        Ok(())
    }
}
