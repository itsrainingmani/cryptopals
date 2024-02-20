use std::{
    collections::HashMap,
    ops::{BitAnd, BitOr, BitXor},
};

pub fn char_frequencies() -> HashMap<u8, f64> {
    HashMap::from([
        (32, 0.167564443682168),
        (101, 0.08610229517681191),
        (116, 0.0632964962389326),
        (97, 0.0612553996079051),
        (110, 0.05503703643138501),
        (105, 0.05480626188138746),
        (111, 0.0541904405334676),
        (115, 0.0518864979648296),
        (114, 0.051525029341199825),
        (108, 0.03218192615049607),
        (100, 0.03188948073064199),
        (104, 0.02619237267611581),
        (99, 0.02500268898936656),
        (10, 0.019578060965172565),
        (117, 0.019247776378510318),
        (109, 0.018140172626462205),
        (112, 0.017362092874808832),
        (102, 0.015750347191785568),
        (103, 0.012804659959943725),
        (46, 0.011055184780313847),
        (121, 0.010893686962847832),
        (98, 0.01034644514338097),
        (119, 0.009565830104169261),
        (44, 0.008634492219614468),
        (118, 0.007819143740853554),
        (48, 0.005918945715880591),
        (107, 0.004945712204424292),
        (49, 0.004937789430804492),
        (83, 0.0030896915651553373),
        (84, 0.0030701064687671904),
        (67, 0.002987392712176473),
        (50, 0.002756237869045172),
        (56, 0.002552781042488694),
        (53, 0.0025269211093936652),
        (65, 0.0024774830020061096),
        (57, 0.002442242504945237),
        (120, 0.0023064144740073764),
        (51, 0.0021865587546870337),
        (73, 0.0020910417959267183),
        (45, 0.002076717421222119),
        (54, 0.0019199098857390264),
        (52, 0.0018385271551164353),
        (55, 0.0018243295447897528),
        (77, 0.0018134911904778657),
        (66, 0.0017387002075069484),
        (34, 0.0015754276887500987),
        (39, 0.0015078622753204398),
        (80, 0.00138908405321239),
        (69, 0.0012938206232079082),
        (78, 0.0012758834637326799),
        (70, 0.001220297284016159),
        (82, 0.0011037374385216535),
        (68, 0.0010927723198318497),
        (85, 0.0010426370083657518),
        (113, 0.00100853739070613),
        (76, 0.0010044809306127922),
        (71, 0.0009310209736100016),
        (74, 0.0008814561018445294),
        (72, 0.0008752446473266058),
        (79, 0.0008210528757671701),
        (87, 0.0008048270353938186),
        (106, 0.000617596049210692),
        (122, 0.0005762708620098124),
        (47, 0.000519607185080999),
        (60, 0.00044107665296153596),
        (62, 0.0004404428310719519),
        (75, 0.0003808001912620934),
        (41, 0.0003314254660634964),
        (40, 0.0003307916441739124),
        (86, 0.0002556203680692448),
        (89, 0.00025194420110965734),
        (58, 0.00012036277683200988),
        (81, 0.00010001709417636208),
        (90, 8.619977698342993e-05),
        (88, 6.572732994986532e-05),
        (59, 7.41571610813331e-06),
        (63, 4.626899793963519e-06),
        (127, 3.1057272589618137e-06),
        (94, 2.2183766135441526e-06),
        (38, 2.0282300466689395e-06),
        (43, 1.5211725350017046e-06),
        (91, 6.97204078542448e-07),
        (93, 6.338218895840436e-07),
        (36, 5.070575116672349e-07),
        (33, 5.070575116672349e-07),
        (42, 4.436753227088305e-07),
        (61, 2.5352875583361743e-07),
        (126, 1.9014656687521307e-07),
        (95, 1.2676437791680872e-07),
        (9, 1.2676437791680872e-07),
        (123, 6.338218895840436e-08),
        (64, 6.338218895840436e-08),
        (5, 6.338218895840436e-08),
        (27, 6.338218895840436e-08),
        (30, 6.338218895840436e-08),
    ])
}

pub fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 == 0 {
        (0..s.len())
            .step_by(2)
            .map(|i| {
                s.get(i..i + 2)
                    .and_then(|sub| u8::from_str_radix(sub, 16).ok())
            })
            .collect()
    } else {
        None
    }
}

pub fn bytes_to_hex(v: Vec<u8>) -> String {
    v.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn score(v: &Vec<u8>) -> f64 {
    // let mut score = 0.;
    // let alphabet = "abcdefghijklmnopqrstuvwxyz";

    // for c in v.chars() {
    //     if alphabet.contains(c.to_ascii_lowercase()) {
    //         score += 1.;
    //     }
    // }

    let mut hist: HashMap<u8, f64> = HashMap::new();

    for b in v.iter() {
        if hist.contains_key(b) {
            hist.get_mut(&b).map(|val| *val += 1.0 / v.len() as f64);
        } else {
            hist.insert(*b, 1.0 / v.len() as f64);
        }
    }

    let mut score: f64 = 0.;
    let char_freq = char_frequencies();

    for b in v.iter() {
        if char_freq.contains_key(b) {
            score += hist[&b] * (hist[&b] / char_freq[&b]).ln();
        } else {
            score += hist[&b] * (hist[&b] / 1e-10).ln();
        }
    }

    score
}

pub fn hamming(s1: &[u8], s2: &[u8]) -> u32 {
    // if the strings have different lengths, then we start the hamming distance with that difference
    // Bitwise XOR is 1 if the bits are different and 0 if the bits are the same

    s1.iter()
        .zip(s2.iter())
        .fold((s1.len().abs_diff(s2.len())) as u32, |sum: u32, (a, b)| {
            sum + a.bitxor(b).count_ones()
        })
}

pub fn into_blocks(ciphertext: &Vec<u8>, keysize: usize) -> Vec<Vec<u8>> {
    let mut blocks_vec = Vec::new();

    for c in ciphertext.chunks(keysize) {
        if c.len() != keysize {
            let mut padded = Vec::from(c);
            padded.extend(vec![0; keysize - c.len()]);
            blocks_vec.push(padded);
        } else {
            blocks_vec.push(c.to_vec());
        }
    }

    blocks_vec
}

pub fn transpose_blocks(blocks: Vec<Vec<u8>>, keysize: usize) -> Vec<Vec<u8>> {
    let mut transposed = Vec::new();

    for i in 0..keysize {
        let transpo_block = blocks
            .iter()
            .map(|v| *v.get(i).unwrap())
            .collect::<Vec<u8>>();

        transposed.push(transpo_block);
    }

    transposed
}

pub fn xor_single(i: &Vec<u8>, by: u8) -> Vec<u8> {
    i.iter().map(|x| *x ^ by).collect::<Vec<u8>>()
}

pub fn pkcs7_padding(block: String, block_length: usize) -> String {
    // Given a block, pad the boock to a specific block length but appending the number of
    // bytes of padding to the end of the block.

    let mut block_bytes = block.into_bytes();

    let remainder = block_length.rem_euclid(block_bytes.len());

    let mut padding: Vec<u8> = Vec::new();

    for _ in 0..remainder {
        // let single_padding_byte = format!("{:#x}", remainder as u8);
        padding.push(remainder as u8);
    }

    block_bytes.append(&mut padding);

    String::from_utf8(block_bytes).unwrap()
}

mod tests {
    #[test]
    fn test_single_xor() {
        let input =
            Vec::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let xored = super::xor_single(&input, 88);

        // println!("{}", String::from_utf8(xored.clone()).unwrap());

        for i in input.iter().zip(xored) {
            println!("{} ^ 88 - {}", i.0, i.1);
        }
    }

    #[test]
    fn test_hamming_distance() {
        let dist = super::hamming("this is a test".as_bytes(), "wokka wokka!!!".as_bytes());
        assert_eq!(dist, 37);
    }

    #[test]
    fn test_block_generation() {
        let ciphertext: Vec<u8> = vec![0, 0, 1, 2, 3];
        let blocks = super::into_blocks(&ciphertext, 4);
        assert_eq!(blocks, vec![vec![0, 0, 1, 2], vec![3, 0, 0, 0]]);
    }

    #[test]
    fn test_block_transpose() {
        let ciphertext: Vec<u8> = vec![0, 0, 1, 2, 3, 4, 5, 6];
        let blocks = super::into_blocks(&ciphertext, 4);
        let transposed = super::transpose_blocks(blocks, 4);
        // println!("{:?}", transposed);
        assert_eq!(
            transposed,
            vec![vec![0, 3], vec![0, 4], vec![1, 5], vec![2, 6]]
        );
    }

    #[test]
    fn test_pkcs7_padding() {
        let input_block = String::from("YELLOW SUBMARINE");
        let padded_length: usize = 20;

        // println!(
        //     "{}",
        //     crate::utils::pkcs7_padding(input_block, padded_length)
        // );

        assert_eq!(
            "YELLOW SUBMARINE\x04\x04\x04\x04",
            crate::utils::pkcs7_padding(input_block, padded_length)
        )
    }
}
