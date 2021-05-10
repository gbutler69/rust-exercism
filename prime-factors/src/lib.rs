use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
    unimplemented,
};

pub fn factors(number_to_factor: u64) -> Vec<u64> {
    lazy_static! {
        static ref PRIME_COMPUTER: Primes = Primes::new(100);
    }
    PRIME_COMPUTER.find_or_compute_factors(number_to_factor)
}

struct Primes {
    factorized: Arc<Mutex<HashMap<u64, Vec<u64>>>>,
    primes: Arc<Mutex<Vec<u64>>>,
}

impl Primes {
    pub fn new(reservation_for_primes_size: usize) -> Self {
        Primes {
            factorized: Arc::new(Mutex::new(HashMap::new())),
            primes: Arc::new(Mutex::new(Self::hardcoded_precomputed_primes(
                reservation_for_primes_size,
            ))),
        }
    }

    fn hardcoded_precomputed_primes(reservation_for_primes_size: usize) -> Vec<u64> {
        let reservation_for_primes_size = (reservation_for_primes_size + 1).max(10);
        let mut primes = Vec::<u64>::with_capacity(reservation_for_primes_size);
        primes.append(&mut vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
        primes.resize(reservation_for_primes_size, 0);
        primes
    }

    pub fn find_or_compute_factors(&self, number_to_factor: u64) -> Vec<u64> {
        if let Some(factors) = self.retrieve_stored_factors(number_to_factor) {
            return factors;
        }
        self.compute_and_store_factors(number_to_factor)
    }

    fn retrieve_stored_factors(&self, number_to_factor: u64) -> Option<Vec<u64>> {
        if let Ok(factors) = self.factorized.clone().lock() {
            if let Some(factors) = factors.get(&number_to_factor) {
                return Some(factors.clone());
            }
        }
        None
    }

    fn compute_and_store_factors(&self, number_to_factor: u64) -> Vec<u64> {
        let approx_max_factors = (number_to_factor as f64).log10() as u64 + 10;
        let mut factors = Vec::with_capacity(approx_max_factors as usize);
        let mut n = number_to_factor;
        for i in 0..=number_to_factor {
            let prime_i = self.compute_nth_prime(i);
            while n % prime_i == 0 {
                factors.push(prime_i);
                n /= prime_i;
            }
            if n == 1 || prime_i > n {
                break;
            }
        }
        if let Ok(mut factorized) = self.factorized.clone().lock() {
            factorized.insert(number_to_factor, factors.clone());
        }
        factors
    }

    pub fn compute_nth_prime(&self, n: u64) -> u64 {
        let n = n as usize;
        match self.primes.clone().lock() {
            Ok(primes) => Self::find_or_compute_nth_prime(primes, n),
            Err(_) => unimplemented!(
                "pre-computed primes mutex poisoned - a thread panicked while holding the lock"
            ),
        }
    }

    fn find_or_compute_nth_prime(mut primes: MutexGuard<Vec<u64>>, n: usize) -> u64 {
        let min_primes_size = (n + 1).max(primes.capacity());
        primes.resize(min_primes_size, 0);
        let mut current_prime = 0_u64;
        for p_idx in 0..=n {
            if primes[p_idx] != 0 {
                current_prime = primes[p_idx];
            } else {
                Self::compute_next_prime(current_prime, p_idx, &mut *primes);
                current_prime = primes[p_idx];
            }
        }
        primes[n]
    }

    fn compute_next_prime(current_prime: u64, prime_idx_to_compute: usize, primes: &mut [u64]) {
        for next_prime_candidate in ((current_prime + 2)..u64::MAX).step_by(2) {
            let prev_primes = primes.get(0..prime_idx_to_compute).unwrap();
            if Self::is_prime(next_prime_candidate, prev_primes) {
                primes[prime_idx_to_compute] = next_prime_candidate;
                break;
            }
        }
    }

    fn is_prime(next_prime_candidate: u64, prev_primes: &[u64]) -> bool {
        let prime_candidate_sqrt = (next_prime_candidate as f64).sqrt() as u64 + 1;
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
}
