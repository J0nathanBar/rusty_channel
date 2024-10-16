#[derive(Clone)]
pub struct FecConfig {
    pub source_symbols_num: usize,
    pub nb_repair: u32,
    pub data_length: usize,
}
impl FecConfig {
    pub fn new(source_symbols_num: usize, nb_repair: u32, data_length: usize) -> FecConfig {
        FecConfig {
            source_symbols_num,
            nb_repair,
            data_length,
        }
    }
}
