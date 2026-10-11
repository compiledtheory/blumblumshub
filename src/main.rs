mod structs;
mod cypher;
mod bbs;
mod nist;

use structs::{CypherData, Secret, TestAccumulation};
use rand::Rng;


fn main() {
    let numbers = bbs::generate_biguint_struct();
    let bbs = bbs::generate(&numbers);
    let rng = rand::thread_rng();
    let n = bbs.n();

    let streams = 30;
    let accumulation = test_multiple_streams(&numbers, rng, &n, streams);

    accumulation.results(streams);

    let secret = Secret::from_file("source_text.txt");
    let secret_bytes = secret.msg.as_bytes();
    let secret_bits = secret.bits();
    let bits = secret_bits.max(100_000);

    let s = bbs::generate_s(&n, &numbers);
    let cypher_data = CypherData{bits, n, s};

    let bbs_bits = bbs::make_bbs_bits(cypher_data, &numbers);
    let test_results = nist::run_test(&bbs_bits, 0.01);
    println!("{test_results}");
    let keystream_bytes = cypher::make_keystream_bytes(bbs_bits);
    
    let encrypted = cypher::vernan_cypher(secret_bytes, &keystream_bytes);
    let decrypted = cypher::vernan_cypher(&encrypted, &keystream_bytes);

    println!("Decryption yielded same result as message?: {}", secret_bytes == decrypted);
}

fn test_multiple_streams(numbers: &structs::Numbers, mut rng: rand::prelude::ThreadRng, n: &num_primes::BigUint, streams: u32) -> TestAccumulation {
    let mut accumulation = TestAccumulation::new();
    
    for i in 0..streams {
        let bits = rng.gen_range(100_000..=110_000);
        let s = bbs::generate_s(n, numbers);
        let cypher_data = CypherData{bits, n: n.clone(), s};
        let bbs_bits = bbs::make_bbs_bits(cypher_data, numbers);
        let result = nist::run_test(&bbs_bits, 0.01);
        accumulation.update(&result);
        println!("Iteration {i} results on {bits} bits: ");
        println!("{result}");
    
    }
    return accumulation;
    }