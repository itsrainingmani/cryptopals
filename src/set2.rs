use crate::utils::*;
use base64::prelude::*;

// Implement PKCS7 Padding
//
pub fn challenge9() {
    let input = String::from("YELLOW SUBMARINE");
    let block_size: usize = 20;

    println!("{}", pkcs7_padding(input, block_size));
}

/*
Implement CBC Mode
CBC mode is a block cipher mode that allows us to encrypt irregularly-sized messages, despite the fact that a block cipher natively only transforms individual blocks.
In CBC mode, each ciphertext block is added to the next plaintext block before the next call to the cipher core.
*/
pub fn challenge10() {}
