use num_primes::{BigUint, Generator};
use crate::structs;

// Wygenerowanie losowej liczby pierwszej uwzgledniajac test Rabin-Miller
// oraz weryfikacja modulo 4

pub fn generate(numbers: &structs::Numbers) -> structs::BlumBlumShub {
    let p = get_blum_prime(&numbers);
    let q = loop {
        let q = get_blum_prime(&numbers);

        // odrzucenie przypadku gdy p == q
        if !(&p == &q) {
            break q;
        }
    };

    return structs::BlumBlumShub{p, q};    
}

fn get_blum_prime(numbers: &structs::Numbers) -> BigUint {
    let num = loop {
        let num = Generator::new_prime(1024);
        
        if (&num % &numbers.four) == numbers.three {
            break num;
        }
    };

    return num;
}

pub fn make_bbs_bits(cypher_data: structs::CypherData, numbers: &structs::Numbers) -> Vec<u8> {
    let mut x = cypher_data.s.modpow(&numbers.two, &cypher_data.n);
    let mut bbs_bits: Vec<u8> = Vec::with_capacity(cypher_data.bits);

    for _ in 0..cypher_data.bits {
        x = x.modpow(&numbers.two, &cypher_data.n);
        let lsb = if (&x % &numbers.two) == numbers.one {1u8} else {0u8};
        bbs_bits.push(lsb);
    }

    return bbs_bits;
}

pub fn generate_s(n: &BigUint, numbers: &structs::Numbers) -> BigUint {
    let bit_size = n.bits();
    let candidate = loop {
        let candidate = Generator::new_uint(bit_size);
        
        // czy liczba jest w przedziale <1, n - 1)
        if !(candidate >= numbers.one && candidate < *n) {
            continue;
        }

        // czy greatest common divisor == 1
        if gcd(candidate.clone(), n.clone(), numbers) == numbers.one {
            break candidate;
        }
    };

    return candidate;
}

pub fn gcd(mut a: BigUint, mut b: BigUint, numbers: &structs::Numbers) -> BigUint {
    while b != numbers.zero {
        let temp = b.clone();
        b = a % &b;
        a = temp;
    }
    return a;
}