pub fn nth(n: u32) -> u32 {
    let n = n as usize;
    let mut primes: Vec<u32> = initialize_precomputed_primes(n);
    let mut current_prime = 0_u32;
    for p_idx in 0..=n {
        if primes[p_idx] != 0 {
            current_prime = primes[p_idx];
        } else {
            compute_next_prime(current_prime, p_idx, &mut primes);
            current_prime = primes[p_idx];
        }
    }
    primes[n]
}

fn initialize_precomputed_primes(n: usize) -> Vec<u32> {
    let n = (n + 1).max(10);
    let mut primes = Vec::<u32>::with_capacity(n);
    primes.append(&mut vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    primes.resize(n, 0);
    primes
}

fn compute_next_prime(current_prime: u32, prime_idx_to_compute: usize, primes: &mut Vec<u32>) {
    for next_prime_candidate in ((current_prime + 2)..u32::MAX).step_by(2) {
        let prev_primes = primes.get(0..prime_idx_to_compute).unwrap();
        if is_prime(next_prime_candidate, prev_primes) {
            primes[prime_idx_to_compute] = next_prime_candidate;
            break;
        }
    }
}

fn is_prime(next_prime_candidate: u32, prev_primes: &[u32]) -> bool {
    let prime_candidate_sqrt = (next_prime_candidate as f64).sqrt() as u32 + 1;
    for &factor in prev_primes {
        if factor >= prime_candidate_sqrt {
            return true;
        }
        if next_prime_candidate % factor == 0 {
            return false;
        }
    }
    return true;
}
