pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    eliminate_composites_using_sieve_of_eratosthenes_up_to(upper_bound)
        .into_iter()
        .enumerate()
        .filter_map(only_if_prime)
        .collect()
}

fn eliminate_composites_using_sieve_of_eratosthenes_up_to(upper_bound: u64) -> Vec<bool> {
    let mut maybe_primes = vec![true; (upper_bound - 1) as usize];
    let mut prime_to_test = 2_usize;
    while prime_to_test < upper_bound as usize {
        prime_to_test =
            match eliminate_composites_factorable_by_this_prime_and_return_next_prime_to_test(
                prime_to_test,
                &mut maybe_primes,
            ) {
                Some(p) => p,
                None => break,
            }
    }
    maybe_primes
}

fn eliminate_composites_factorable_by_this_prime_and_return_next_prime_to_test(
    prime_to_test: usize,
    maybe_primes: &mut Vec<bool>,
) -> Option<usize> {
    maybe_primes
        .iter_mut()
        .skip(prime_to_test - 2)
        .step_by(prime_to_test)
        .skip(1)
        .for_each(|el| *el = false);
    maybe_primes
        .iter()
        .enumerate()
        .skip(prime_to_test - 1)
        .filter_map(|(idx, &is_prime)| match is_prime {
            true => Some(idx + 2),
            false => None,
        })
        .nth(0)
}

fn only_if_prime((idx, is_prime): (usize, bool)) -> Option<u64> {
    match is_prime {
        true => Some(idx as u64 + 2),
        false => None,
    }
}
