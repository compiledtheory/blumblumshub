use std::f32::consts::SQRT_2;
use statrs::function::gamma::gamma_ur;

use crate::structs;




pub fn run_test(bits: &[u8], alfa: f32) -> structs::TestResults {
    let monobit = frequency_monobit_test(bits);
    let block = frequency_test_within_a_block(bits);
    let runs = runs_test(bits);
    let longest_ones =longest_run_ones_in_block(bits);
    let serial = serial_test(bits);

    return structs::TestResults{
        monobit: (monobit >= alfa),
        block: (block >= alfa),
        runs: (runs >= alfa),
        longest_ones: (longest_ones >= alfa),
        serial_1: (serial.0 >= alfa),
        serial_2: (serial.1 >= alfa),
    }
}

fn frequency_monobit_test(bits: &[u8]) -> f32 {
    let mut sum: i32 = 0;
    for bit in bits {
        match bit {
            2_u8..=u8::MAX => continue,
            1 => sum += 1,
            0 => sum -= 1,
        }
    }

    let n = bits.len() as f32;
    let sum = sum as f32;

    let s_obs: f32 = sum.abs() / n.sqrt();
    return libm::erfcf(s_obs / SQRT_2);
}

fn frequency_test_within_a_block(bits: &[u8]) -> f32 {
    let chunk_size = 128;

    let n_blocks = (bits.len() / chunk_size) as f64;
    let m_f64 = chunk_size as f64;

    let mut chi = 0.0;
    for chunk in bits.chunks_exact(chunk_size) {
        let ones: usize = chunk.iter().map(|&b| b as usize).sum();
        let pi = (ones as f64) / m_f64;
        chi += (pi - 0.5).powi(2);
    }

    chi *=  4.0 * m_f64;
    return gamma_ur(n_blocks / 2.0 , chi / 2.0) as f32;
}

fn runs_test(bits: &[u8]) -> f32 {
    let n = bits.len() as f32;
    let ones: usize = bits.iter().map(|&b| b as usize).sum();
    let pi = (ones as f32) / n;

    let tau = 2.0 / n.sqrt();

    if (pi - 0.5).abs() >= tau {
        return 0.0;
    }

    let mut v_obs = 1.0;
    for i in 0..(bits.len() - 1) {
        let bit = bits[i];
        let next = bits[i+1];
        if bit != next {
            v_obs += 1.0;
        }
    }

    let expected_runs = 2.0 * n * pi * (1.0 - pi);
    let numerator = (v_obs - expected_runs).abs();
    let denominator = 2.0 * (2.0 * n).sqrt() * pi * (1.0 - pi);

    return libm::erfcf(numerator / denominator);
}

fn longest_run_ones_in_block(bits: &[u8]) -> f32 {
    let n = bits.len();
    let chunk_size = 128;
    let blocks = (n / chunk_size) as f64;

    let mut v = [0.0_f64; 6];
    for block in bits.chunks_exact(chunk_size) {
        let mut max_run = 0;
        let mut curr_run = 0;
        for &bit in block {
            match bit == 1{
                true => {
                    curr_run += 1;
                    if curr_run > max_run {
                        max_run = curr_run;
                    }
                }
                false => {curr_run = 0;}
            }
        }

        let index = match max_run {
            0..=4 => 0,
            5 => 1,
            6 => 2,
            7 => 3,
            8 => 4,
            _ => 5,
        };

        v[index] += 1.0;
    }

    let pi = [0.1174, 0.2430, 0.2493, 0.1752, 0.1027, 0.1124];
    let mut chi_square: f64 = 0.0;

    for i in 0..6 {
        let expected = blocks * pi[i];
        chi_square += (v[i] - expected).powi(2) / expected;
    }

    return gamma_ur(5.0 / 2.0, chi_square / 2.0) as f32
}

fn serial_test(bits: &[u8]) -> (f32, f32) {
    let n = bits.len() as f64;
    let m = 5;

    let mut augmented: Vec<u8> = bits.to_vec();

    augmented.extend_from_slice(&bits[0..m-1]);

    let counts_m = count_window(m, &augmented);
    let counts_m1 = count_window(m - 1, &augmented[0..augmented.len() - 1]);
    let counts_m2 = count_window(m - 2, &augmented[0..augmented.len() - 2]);

    let psi_m = count_psi(n, m, &counts_m);
    let psi_m1 = count_psi(n, m - 1, &counts_m1);
    let psi_m2 = count_psi(n, m - 2, &counts_m2);

    let delta_1 = psi_m - psi_m1;
    let delta_2 = psi_m - 2.0 * psi_m1 + psi_m2;

    let p1 = gamma_ur((1 << (m - 2)) as f64, delta_1 / 2.0) as f32;
    let p2 = gamma_ur((1 << (m - 3)) as f64, delta_2 / 2.0) as f32;

    return (p1, p2);
 }

fn count_window(window_size: usize, bits: &[u8]) -> Vec<f64> {
    let patterns = 1 << window_size;
    let mut counts = vec![0.0_f64; patterns];

    for window in bits.windows(window_size) {
        let mut val = 0;
        for &bit in window {
            val = (val << 1) | (bit as usize);
        }
        counts[val] += 1.0;
    }

    return counts;
}

fn count_psi(n: f64, m: usize, bits: &[f64]) -> f64 {
    let two_m = ((1 << m) as f64) / n;
    let sum_sq: f64 = bits.iter().map(|&c| c * c).sum();
    return two_m * sum_sq - n;
}