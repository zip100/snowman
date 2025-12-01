use hex;
use rand_core::{OsRng, RngCore};
use sha2::{Digest, Sha256};

mod bip39_words;
fn main() {
    let entropy = genrate_entropy();
    println!("Entropy: {}", hex::encode(entropy));

    let checksum = hash_entropy(entropy);
    println!("Checksum: {}", hex::encode(checksum.to_le_bytes()));

    let mut bits = entropy.to_vec();
    bits.push((checksum as u8) << 4);
    print!("Bits: {}", bits.len());

    let mut words = Vec::new();
    for i in (0..bits.len() * 8).step_by(11) {
        let mut word: u16 = 0u16;
        for j in 0..11 {
            if i + j < bits.len() * 8 {
                let index = (i + j) / 8;
                let offset = 7 - (i + j) % 8;
                let bit = (bits[index] >> offset) & 1;
                word = (word << 1) | bit as u16;
            }
        }
        if word > 0 {
            words.push(word);
        }
    }
    println!("Words: {:?}", words);

    for i in 0..words.len() {
        let word = words[i];
        let word_str = bip39_words::get_bip39_word(word as usize).unwrap();
        println!("Word {}: {}", i, word_str);
    }
}

fn genrate_entropy() -> [u8; 16] {
    let mut entropy = [0u8; 16];
    OsRng.fill_bytes(&mut entropy);
    entropy
}

fn hash_entropy(entropy: [u8; 16]) -> u8 {
    Sha256::digest(entropy)[0] >> 4
}
