# Blum Blum Shub PRNG & Vernam Cipher Pipeline

A full-stack cryptographic pipeline and data visualization suite. This project implements a Blum Blum Shub (BBS) pseudo-random number generator (PRNG) to generate a secure keystream, statistically validates the bitstream's randomness using the NIST SP 800-22 test suite, and applies the verified keystream to encrypt and decrypt a file using a Vernam cipher (One-Time Pad). 

The project is structured as a monorepo leveraging Rust for high-performance cryptographic operations and Python for data analysis and plotting.

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
  * Mathematical implementations utilize upper regularized incomplete gamma functions and complementary error functions.
* **Vernam Cipher Encryption (XOR):**
  * Secure byte-packing of the PRNG bitstream into a usable keystream.
  * Guaranteed secure encryption and decryption assuming a single-use keystream.
* **Automated Data Visualization (Python):**
  * Object-oriented Matplotlib pipeline (`dataclasses`) for generating publication-ready plots.
  * Outputs Bar Charts, Heatmaps, and Scatter Plots of the NIST test results.

## Project Structure

* `src/` - Rust codebase orchestrating the cryptographic pipeline.
  * `main.rs` - Parameter initialization, 30-stream NIST validation, and practical text encryption.
  * `bbs.rs` - BigUint math logic for prime/seed generation and PRNG sequence extraction.
  * `nist.rs` - Mathematical implementations of the statistical test formulas.
  * `cypher.rs` - Vernam encryption/decryption via XOR and bit-to-byte packing.
  * `structs.rs` - Type definitions and custom `Display` trait implementations.
* `data/` - Output directory where the Rust program exports cryptographic parameters (`report_data.txt`) and NIST test results (`results.csv`).
* `analysis/` - Python data analysis module.
  * `main.py` - Ingestion and configuration script.
  * `graphs.py` - Matplotlib rendering functions.
  * `structs.py` - Dataclass configurations for decoupled presentation logic.
* `source_text.txt` - The plaintext input file used for encryption.

## Prerequisites

* **Rust:** `cargo` and `rustc` (Edition 2021)
* **Python:** Python 3.8+ (with `venv` support)

## Usage

### Step 1: Cryptographic Pipeline (Rust)
Clone the repository and modify `source_text.txt` to add the message you wish to encrypt.

```bash
git clone [https://github.com/compiledtheory/blumblumshub.git](https://github.com/compiledtheory/blumblumshub.git)
cd blumblumshub
cargo run --release
```

**Generated Artifacts:**
The Rust binary will generate `data/results.csv` containing the boolean pass/fail results of the NIST tests across all streams, and `data/report_data.txt` containing the BBS parameters, random seed, and ciphertext snippets.

### Step 2: Data Visualization (Python)
Navigate to the analysis folder, set up your virtual environment, and run the plotting script.

```bash
cd analysis
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
python main.py
```

**Generated Artifacts:**
The Python script will output high-resolution PNGs into `analysis/output/`:
* `nist_pass_rates.png`: Bar chart of overall pass percentages.
* `pass_fail_matrix.png`: Heatmap pinpointing specific stream failures.
* `stream_lengths.png`: Scatter plot documenting exact bit lengths.

## Sample Rust Output

```text
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
```