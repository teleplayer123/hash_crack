use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crypto_hashes;

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

    let mut hasher = match hash_alg {
        "sha256" => crypto_hashes::sha256::Sha256::new(),
        "sha512" => crypto_hashes::sha512::Sha512::new(),
        "md5" => crypto_hashes::md5::Md5::new(),
        _ => {
            eprintln!("Unsupported hash algorithm: {}", hash_alg);
            return;
        }
    };
}
