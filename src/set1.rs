use crate::utils::{self, xor_single};
use base64::prelude::*;

pub fn challenge1() {
    let hex_string = utils::hex_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
    println!("{:?}", BASE64_STANDARD.encode(hex_string));
}

pub fn challenge2() {
    let s1 = utils::hex_to_bytes("1c0111001f010100061a024b53535009181c").unwrap();
    let s2 = utils::hex_to_bytes("686974207468652062756c6c277320657965").unwrap();

    let xored_bytes: Vec<u8> = s1.iter().zip(s2.iter()).map(|(b1, b2)| *b1 ^ *b2).collect();

    println!("{}", String::from_utf8(xored_bytes).unwrap());
}

pub fn challenge3() {
    let decoded =
        utils::hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
            .unwrap();
    let mut high_score = (1000000000., vec![], 0);

    for ord in 32..=126 {
        let xored_bytes = xor_single(&decoded, ord as u8);
        let cur_score = utils::score(&xored_bytes);

        if cur_score < high_score.0 {
            high_score = (cur_score, xored_bytes, ord as u8);
        }
    }

    println!(
        "{} | {} | {}",
        high_score.0,
        String::from_utf8(high_score.1).unwrap(),
        high_score.2
    );
}

pub fn challenge4() {
    let input_file = std::fs::read_to_string("inputs/challenge4.txt").unwrap();

    let mut high_score = (1000000., vec![]);
    for line in input_file.split('\n') {
        for ord in 32..=126 {
            let decoded = utils::hex_to_bytes(line).unwrap();
            let xored_string = xor_single(&decoded, ord as u8);
            // println!("{}", xored_string);
            let cur_score = utils::score(&xored_string);

            if cur_score < high_score.0 {
                high_score = (cur_score, xored_string.clone());
            }
        }
    }

    println!(
        "{} | {}",
        high_score.0,
        String::from_utf8(high_score.1).unwrap()
    );
}

// Repeating key XOR
pub fn challenge5() {
    let phrase = r#"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal"#;

    let key = "ICE";

    let mut encrypted_xor: Vec<u8> = Vec::new();

    for (i, j) in phrase.bytes().zip(key.bytes().cycle()) {
        encrypted_xor.push(i ^ j);
    }

    println!("{}", utils::bytes_to_hex(encrypted_xor));
}
