use crate::FecConfig;



#[cfg(test)]
mod tests {
    use super::*;

    fn fec_enc(data: &Vec<u8>, config: &FecConfig) -> Vec<Vec<u8>> {
        let mut retval = Vec::with_capacity(config.source_symbols + config.nb_repair as usize);
        let mut encoder = raptor_code::SourceBlockEncoder::new(&data[..], config.source_symbols);
        let n = encoder.nb_source_symbols() + config.nb_repair;
        for esi in 0..n as u32 {
            let symbol = encoder.fountain(esi);
            retval.push(symbol);
        }
        retval
    }
    
    fn fec_dec(data: Vec<Vec<u8>>, config: &FecConfig) -> Vec<u8> {
        let source_block_length = config.data_length;
        let mut n = 0u32;
        let mut decoder = raptor_code::SourceBlockDecoder::new(config.source_symbols);
        
        while !decoder.fully_specified() {
            decoder.push_encoding_symbol(&data[n as usize], n);
            n += 1;
        }
        decoder.decode(source_block_length).unwrap()
    }
    
    #[test]
    fn it_works() {
        let og_data = String::from("i like to eat stuff because stuff is yummy and good for me yeet");
        let clone = og_data.clone();
        let raw_data = clone.into_bytes();
        let mut config = FecConfig {
            data_length: raw_data.len(),
            source_symbols: 4,
            nb_repair: 3,
        };
        let encoded = fec_enc(&raw_data, &mut config);
        let decoded = fec_dec(encoded, &config);
        let result = String::from_utf8(decoded).unwrap();
        assert_eq!(og_data, result);
    }
}
