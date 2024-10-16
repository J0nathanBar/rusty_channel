pub mod fec_tests;

pub struct FecConfig {
    pub source_symbols: usize,
    pub nb_repair: u32,
    pub data_length: usize,
}
// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
