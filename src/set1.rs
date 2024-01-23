use std::collections::HashSet;

use crate::utils::{self, xor_single};
use base64::prelude::*;
use openssl::symm::{decrypt, Cipher};

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

// Break repeating-key XOR
pub fn challenge6() {
    let base64_input = std::fs::read_to_string("inputs/challenge6.txt")
        .unwrap()
        .replace("\n", "");

    let input = BASE64_STANDARD.decode(base64_input).unwrap();
    println!("Number of Bytes - {}", input.len());

    let mut normed_keysizes: Vec<(usize, f32)> = vec![];

    // For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes, and find the edit distance between them. Normalize this result by dividing by KEYSIZE.
    for keysize in 2..=40 {
        let keyblocks = [&input[0..keysize], &input[keysize..(keysize * 2)]];

        let mut hamming_distances = Vec::new();
        for blk in keyblocks {
            for chunk in input.chunks(keysize) {
                hamming_distances.push(utils::hamming(chunk, blk));
            }
        }

        let avg_dist =
            hamming_distances.iter().sum::<u32>() as f32 / hamming_distances.len() as f32;
        let normalized = avg_dist / keysize as f32;

        normed_keysizes.push((keysize, normalized));
    }
    // Unstable in-place sorting using the f32 totally ordered comparator function
    normed_keysizes.sort_unstable_by(|a, b| a.1.total_cmp(&b.1));
    // println!("{:?}", normed_keysizes);

    // let potential_keysizes: Vec<usize> = normed_keysizes[0..3].iter().map(|(k, _e)| *k).collect();
    let potential_key = normed_keysizes.get(0).unwrap().0;
    let mut xor_keys: Vec<u8> = vec![];
    let blocks = utils::into_blocks(&input, potential_key);
    let transposed = utils::transpose_blocks(blocks, potential_key);

    for blk in transposed {
        let mut high_score = (1000000., 0);
        for ord in 32..=126 {
            let xored_bytes = xor_single(&blk, ord as u8);
            let cur_score = utils::score(&xored_bytes);

            if cur_score < high_score.0 {
                high_score = (cur_score, ord as u8);
            }
        }
        xor_keys.push(high_score.1);
    }
    println!("{:?}", String::from_utf8(xor_keys.clone()).unwrap());

    let mut decrypted = Vec::new();
    for (i, j) in input.iter().zip(xor_keys.iter().cycle()) {
        decrypted.push(*i ^ *j);
    }

    println!("{}", String::from_utf8(decrypted).unwrap());
}

pub fn challenge7() {
    let base64_input = std::fs::read_to_string("inputs/challenge7.txt")
        .unwrap()
        .replace("\n", "");

    let data = BASE64_STANDARD.decode(base64_input).unwrap();
    println!("Number of Bytes - {}\n", data.len());

    let cipher = Cipher::aes_128_ecb();
    let key = Vec::from(String::from("YELLOW SUBMARINE"));
    let decrypted = decrypt(cipher, &key, None, &data).unwrap();

    println!("{}", String::from_utf8(decrypted).unwrap());
}

pub fn challenge8() {
    let input = std::fs::read_to_string("inputs/challenge8.txt").unwrap();
    let list_hex_snippets: Vec<&str> = input.trim().split('\n').collect();
    // let mut byte_strings = Vec::new();

    let mut hashed_set: HashSet<Vec<u8>> = HashSet::new();
    let mut idx_of_ecb = 0;
    for (idx, snippet) in list_hex_snippets.iter().enumerate() {
        // byte_strings.push(utils::hex_to_bytes(snippet).unwrap());
        let byte_str = utils::hex_to_bytes(snippet).unwrap();
        for chunk in byte_str.chunks(16) {
            hashed_set.insert(chunk.to_vec());
        }

        if hashed_set.len() != (snippet.len() % 16) {
            idx_of_ecb = idx;
        }

        hashed_set.clear();
    }

    println!(
        "Snippet encrypted with ECB at {} is - \n{}",
        idx_of_ecb, list_hex_snippets[idx_of_ecb]
    );
}
