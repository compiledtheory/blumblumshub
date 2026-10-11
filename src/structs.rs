use num_primes::BigUint;
use std::{fmt, fs};

pub struct BlumBlumShub {
    pub p: BigUint,
    pub q: BigUint,
}

impl BlumBlumShub {
    pub fn n(&self) -> BigUint {
        return &self.p * &self.q;
    }
}
pub struct Numbers {
    pub zero: BigUint,
    pub one: BigUint,
    pub two: BigUint,
    pub three: BigUint,
    pub four: BigUint,
}

pub struct Secret {
    pub msg: String,
}

impl Secret {

    pub fn bits(&self) -> usize {
        return self.msg.as_bytes().len() * 8;
    }

    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path)
            .expect("Should have been able to read the file");

        return Self {msg: content}
    }
}

pub struct CypherData {
    pub bits: usize,
    pub n: BigUint,
    pub s: BigUint,
}

pub struct TestResults {
    pub monobit: bool,
    pub block: bool, 
    pub runs: bool,
    pub longest_ones: bool,
    pub serial_1: bool,
    pub serial_2: bool,
}

impl fmt::Display for TestResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = |b: bool| if b { "PASS" } else { "FAIL" };

        writeln!(f, "Monobit result:      {}", status(self.monobit))?;
        writeln!(f, "Block result:        {}", status(self.block))?;
        writeln!(f, "Runs result:         {}", status(self.runs))?;
        writeln!(f, "Longest ones result: {}", status(self.longest_ones))?;
        writeln!(f, "Serial 1 result:     {}", status(self.serial_1))?;
        writeln!(f, "Serial 2 result:     {}", status(self.serial_2))
    }
}

pub struct TestAccumulation {
    pub monobit: u32,
    pub block: u32,
    pub runs: u32,
    pub longest_ones: u32,
    pub serial_1: u32,
    pub serial_2: u32,
}

impl TestAccumulation {
    pub fn new() -> Self {
        return Self {
            monobit: 0,
            block: 0,
            runs: 0,
            longest_ones: 0,
            serial_1: 0,
            serial_2: 0,
        }
    }

    pub fn update(&mut self, results: &TestResults) {
        self.monobit += results.monobit as u32;
        self.block += results.block as u32;
        self.runs += results.runs as u32;
        self.longest_ones += results.longest_ones as u32;
        self.serial_1 += results.serial_1 as u32;
        self.serial_2 += results.serial_2 as u32;
    }

    pub fn results(&self, denominator: u32) {
        println!("\n--- Final Results (Out of {} streams) ---", denominator);
        println!("Monobit passed: {} / {}", self.monobit, denominator);
        println!("Block passed: {} / {}", self.block, denominator);
        println!("Runs passed: {} / {}", self.runs, denominator);
        println!("Longest ones passed: {} / {}", self.longest_ones, denominator);
        println!("Serial 1 passed: {} / {}", self.serial_1, denominator);
        println!("Serial 2 passed: {} / {}", self.serial_2, denominator);
    }
}