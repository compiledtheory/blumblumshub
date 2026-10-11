# Blum Blum Shub PRNG & Vernam Cipher Pipeline

A full-stack cryptographic pipeline written in Rust. This project implements a Blum Blum Shub (BBS) pseudo-random number generator (PRNG) to generate a secure keystream, statistically validates the bitstream's randomness using the NIST SP 800-22 test suite, and applies the verified keystream to encrypt and decrypt a file using a Vernam cipher (One-Time Pad).

## Features

* **Cryptographically Secure PRNG (Blum Blum Shub):**
  * Custom implementation enforcing Blum primes ($p \equiv 3 \pmod 4$).
  * Mathematically sound seed generation enforcing $s \in [1, n-1]$ and $\gcd(s, n) = 1$.
* **NIST SP 800-22 Statistical Validation:**
  * Evaluates 30 independent streams of at least 100,000 bits each against standard randomness tests:
    * Frequency (Monobit) Test
    * Frequency Test within a Block
    * Runs Test
    * Longest Run of Ones in a Block
    * Serial Test (1 and 2)
* **Vernam Cipher Encryption (XOR):**
  * Secure byte-packing of the PRNG bitstream into a usable keystream.
  * Guaranteed secure encryption and decryption assuming a single-use keystream.

## Project Structure

* `src/main.rs` - Orchestrates the pipeline: parameter initialization, 30-stream NIST validation, and practical text encryption.
* `src/bbs.rs` - BigUint math logic for prime/seed generation and PRNG sequence extraction.
* `src/nist.rs` - Mathematical implementations of the statistical test formulas (utilizing upper regularized incomplete gamma functions and complementary error functions).
* `src/cypher.rs` - Vernam encryption/decryption via XOR and bit-to-byte packing.
* `src/structs.rs` - Type definitions and custom `Display` trait implementations for clean terminal output.

## Usage

Ensure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed. 

1. Clone the repository:
   ```bash
   git clone [https://github.com/compiledtheory/blumblumshub.git](https://github.com/compiledtheory/blumblumshub.git)
   cd blumblumshub

2. Modify source_text.txt to add the message you wish to encrypt

3. cargo run --release

## Sample Output

Iteration 0 results on 106661 bits: 
Monobit result:      PASS
Block result:        PASS
Runs result:         PASS
Longest ones result: PASS
Serial 1 result:     PASS
Serial 2 result:     PASS

[... iterations 1-28 omitted ...]

Iteration 29 results on 101191 bits: 
Monobit result:      PASS
Block result:        PASS
Runs result:         PASS
Longest ones result: PASS
Serial 1 result:     PASS
Serial 2 result:     PASS

--- Final Results (Out of 30 streams) ---
Monobit passed: 30 / 30
Block passed: 30 / 30
Runs passed: 30 / 30
Longest ones passed: 30 / 30
Serial 1 passed: 30 / 30
Serial 2 passed: 29 / 30
--------------------------------------------------------

Decryption yielded same result as message?: true