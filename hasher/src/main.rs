use base64::Engine;
use base64::engine::general_purpose::STANDARD_NO_PAD;
use mimalloc::MiMalloc;
use rayon::prelude::*;
use sha256::digest;
use sha2::Digest;
use sha2::digest::consts::U32;
use sha2::digest::generic_array::GenericArray;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const MAX_CORES: usize = 100;
const PREFIX: [u8; 9] = [b'e', b'r', b'i', b'k', b'r', b'e', b'e', b'd', b'/'];
const PREFIX_LEN: usize = PREFIX.len();
const N: usize = 11 + PREFIX_LEN;

fn n_to_bytes(n: usize) -> [u8; N] {
    let mut out = [0u8; N];
    out[0..PREFIX_LEN].copy_from_slice(&PREFIX);
    let n = STANDARD_NO_PAD.encode_slice(n.to_ne_bytes(), &mut out[PREFIX_LEN..]).unwrap();
    debug_assert_eq!(n, 11);
    out
}

fn hash_string(n: usize) -> String {
    let s = n_to_string(n);
    digest(s)
}

fn n_to_string(n: usize) -> String {
    let b = n_to_bytes(n);
    let s = String::from_utf8_lossy(&b).to_string();
    s
}

fn hash(n: usize) -> GenericArray<u8, U32> {
    let bytes = n_to_bytes(n);
    sha2::Sha256::digest(bytes)
}

fn score(n: usize) -> usize {
    hash_string(n).find(|c| c != '0').unwrap()
}

fn main() {
    // most below from: https://gist.github.com/zdimension/63d3aa5f04573267f423520e481610ed
    let mut global_best = "Z".to_string();
    let mut global_score = 0;
    let mut start = 6_405_000_000_000;

    const BATCH_SIZE: usize = 100_000_0000;
    loop {
        let range = start..start + BATCH_SIZE;

        let begin = chrono::Utc::now();
        let best = range.into_par_iter()
            .with_min_len(BATCH_SIZE / MAX_CORES)
            .min_by_key(|&n| hash(n))
            .unwrap();
        let score = score(best);
        let elapsed = chrono::Utc::now() - begin;
        let string = hash_string(best);
        let hps = BATCH_SIZE as f64 / elapsed.num_milliseconds() as f64 * 1000.0;

        start += BATCH_SIZE;

        print!("{start:>20} @ {:5.2} MH/s -- ", hps / 1e6);

        if string < global_best {
            println!("Best hash in {:>3} : {:<25} -> {}", score, n_to_string(best), string);
            global_best = string;
            global_score = score;
        } else {
            println!("No better hash found (best: {:>3}, global: {:>3}: {})",
                     score, global_score, global_best);
        }
    }
}
