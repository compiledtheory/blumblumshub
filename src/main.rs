mod structs;
mod cypher;
mod bbs;
mod nist;

use structs::{CypherData, Secret, TestAccumulation, Numbers};
use rand::Rng;
use std::fs::{self, File};
use std::io::Write;


fn main() {
    if let Err(e) = fs::create_dir_all("data") {
        println!("Failed to create data directory: {}", e);
        return;
    }
    let numbers = Numbers::new();
    let bbs = bbs::generate(&numbers);
    let rng = rand::thread_rng();
    let n = bbs.n();

    let mut report_file = File::create("data/report_data.txt").expect("Could not create report file");

    writeln!(report_file, "--- Parametry BBS ---").unwrap();
    writeln!(report_file, "Długość p: {} bitów", bbs.p.bits()).unwrap();
    writeln!(report_file, "Długość q: {} bitów", bbs.q.bits()).unwrap();
    writeln!(report_file, "Długość n: {} bitów", n.bits()).unwrap();
    writeln!(report_file, "Wartość p: {}", bbs.p).unwrap();
    writeln!(report_file, "Wartość q: {}", bbs.q).unwrap();
    writeln!(report_file, "Wartość n: {}", n).unwrap();
    writeln!(report_file, "---------------------\n").unwrap();

    let streams = 30;

    let accumulation = test_multiple_streams(&numbers, rng, &n, streams);

    accumulation.results(streams);

    let secret = Secret::from_file("source_text.txt");
    let secret_bytes = secret.msg.as_bytes();
    let secret_bits = secret.bits();

    writeln!(report_file, "--- Szyfrowanie Vernama ---").unwrap();
    writeln!(report_file, "Tekst jawny: {}", secret.msg).unwrap();
    writeln!(report_file, "Długość tekstu w bitach: {}", secret_bits).unwrap();

    let bits = secret_bits.max(100_000);

    let s = bbs::generate_s(&n, &numbers);

    writeln!(report_file, "Wartość początkowa (ziarno s): {}", s).unwrap();

    let cypher_data = CypherData{bits, n, s};

    let bbs_bits = bbs::make_bbs_bits(cypher_data, &numbers);

    let keystream_bytes = cypher::make_keystream_bytes(bbs_bits);
    
    let encrypted = cypher::vernan_cypher(secret_bytes, &keystream_bytes);
    let decrypted = cypher::vernan_cypher(&encrypted, &keystream_bytes);

    let snippet_len = std::cmp::min(20, secret_bytes.len());
    writeln!(report_file, "Fragment klucza (pierwsze {} bajtów): {:?}", snippet_len, &keystream_bytes[..snippet_len]).unwrap();
    writeln!(report_file, "Fragment szyfrogramu (pierwsze {} bajtów): {:?}", snippet_len, &encrypted[..snippet_len]).unwrap();

    let decrypted_msg = String::from_utf8_lossy(&decrypted);
    writeln!(report_file, "Wynik odszyfrowania (tekst): {}", decrypted_msg).unwrap();
    writeln!(report_file, "Decryption yielded same result as message?: {}", secret_bytes == decrypted).unwrap();
}

fn test_multiple_streams(numbers: &structs::Numbers, mut rng: rand::prelude::ThreadRng, n: &num_primes::BigUint, streams: u32) -> TestAccumulation {
    let mut accumulation = TestAccumulation::new();
    let mut csv_file = File::create("data/results.csv").expect("Could not create csv file");
    writeln!(csv_file, "stream_id,bits,monobit,block,runs,longest_ones,serial_1,serial_2").unwrap();

    for i in 0..streams {
        let bits = rng.gen_range(100_000..=110_000);
        let s = bbs::generate_s(n, numbers);
        let cypher_data = CypherData{bits, n: n.clone(), s};
        let bbs_bits = bbs::make_bbs_bits(cypher_data, numbers);
        let result = nist::run_test(&bbs_bits, 0.01);
        accumulation.update(&result);
        writeln!(csv_file,  "{},{},{},{},{},{},{},{}",
            i, bits, 
            result.monobit as u8, 
            result.block as u8, 
            result.runs as u8, 
            result.longest_ones as u8, 
            result.serial_1 as u8, 
            result.serial_2 as u8
        ).expect("Failed to write to CSV");

        println!("Iteration {i} results on {bits} bits:");
        println!("{result}");
    }
    return accumulation;
}