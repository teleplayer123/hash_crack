use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crypto_hashes::digest::Digest;
use crypto_hashes::{md5, sha1, sha2, sha3};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <wordlist> <hash_alg> <hash>", args[0]);
        return;
    }

    let wordlist_file = File::open(&args[1])
        .expect("Failed to open wordlist file");
    let hash_alg = args[2].trim();
    let hash = args[3].trim();
    let reader = BufReader::new(wordlist_file);

    for line in reader.lines() {
        let word = line.expect("Failed to read line").trim().to_string();
        let computed_hash = match hash_alg {
            "md5" => format!("{:x}", md5::Md5::digest(word.as_bytes())),
            "sha1" => format!("{:x}", sha1::Sha1::digest(word.as_bytes())),
            "sha256" => format!("{:x}", sha2::Sha256::digest(word.as_bytes())),
            "sha512" => format!("{:x}", sha2::Sha512::digest(word.as_bytes())),
            "sha3-256" => format!("{:x}", sha3::Sha3_256::digest(word.as_bytes())),
            "sha3-512" => format!("{:x}", sha3::Sha3_512::digest(word.as_bytes())),
            _ => {
                eprintln!("Unsupported hash algorithm: {}", hash_alg);
                return;
            }
        };

        if computed_hash == hash {
            println!("Found matching word: {}", word);
            return;
        }
    }
}
