use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use mimalloc::MiMalloc;
use rayon::prelude::*;
use sha256::digest;
use sha2::Digest;
use sha2::digest::consts::U32;
use sha2::digest::generic_array::GenericArray;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const MAX_CORES: usize = 100;

fn n_to_bytes(n: usize) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[0..5].copy_from_slice(&[b't', b't', b't', b't', b't']);
    let _ = BASE64_STANDARD.encode_slice(n.to_ne_bytes(), &mut out[5..]).unwrap();
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
    let mut global_best = 0;
    let mut start = 0_000_000_000;
    const BATCH_SIZE: usize = 100_000_000;
    loop {
        let range = start..start + BATCH_SIZE;

        let begin = chrono::Utc::now();
        let best = range.into_par_iter().with_min_len(BATCH_SIZE / MAX_CORES).min_by_key(|n| hash(*n)).unwrap();
        let score = score(best);
        let elapsed = chrono::Utc::now() - begin;
        let hps = BATCH_SIZE as f64 / elapsed.num_milliseconds() as f64 * 1000.0;

        start += BATCH_SIZE;

        print!("{start:>20} @ {:5.2} MH/s -- ", hps / 1e6);

        if score > global_best {
            println!("Best hash in {:>3} : {:<25} -> {}", score, n_to_string(best), hash_string(best));
            global_best = score;
        } else {
            println!("No better hash found (best: {:>3}, global: {:>3})", score, global_best);
        }
    }
}
