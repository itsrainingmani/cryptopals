use crate::utils::*;
use base64::prelude::*;

pub fn challenge9() {
    let input = String::from("YELLOW SUBMARINE");
    let block_size: usize = 20;

    println!("{}", pkcs7_padding(input, block_size));
}
