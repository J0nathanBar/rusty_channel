#[cfg(test)]
mod tests {
    use std::thread;

    use tokio::sync::mpsc;

    use crate::{FecConfig, FecEncoder};

    fn fec_enc(data: Vec<u8>, config: &FecConfig) -> Vec<Vec<u8>> {
        let mut retval = Vec::with_capacity(config.source_symbols_num + config.nb_repair as usize);
        let (tx, mut rx) = mpsc::channel(1024);
        let encoder = FecEncoder::new(config.clone(), tx);
        thread::spawn(move || {encoder.encode(data).unwrap();});
        while !rx.is_closed() {
            let (data, _) = rx.blocking_recv().unwrap();
            retval.push(data);
        }
        retval
    }

    fn fec_dec(data: Vec<Vec<u8>>, config: &FecConfig) -> Vec<u8> {
        let source_block_length = config.data_length;
        let mut n = 0u32;
        let mut decoder = raptor_code::SourceBlockDecoder::new(config.source_symbols_num);

        while !decoder.fully_specified() {
            decoder.push_encoding_symbol(&data[n as usize], n);
            n += 1;
        }
        decoder.decode(source_block_length).unwrap()
    }

    #[test]
    fn test_fec() {
        let og_data =
            String::from("i like to eat stuff because stuff is yummy and good for me yeet");
        let clone = og_data.clone();
        let raw_data = clone.into_bytes();
        let config = FecConfig {
            data_length: raw_data.len(),
            source_symbols_num: 4,
            nb_repair: 3,
        };
        let encoded = fec_enc(raw_data, &config);
        let decoded = fec_dec(encoded, &config);
        let result = String::from_utf8(decoded).unwrap();
        assert_eq!(og_data, result);
    }
}
