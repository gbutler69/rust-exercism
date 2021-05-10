use std::time::{Duration, SystemTime};

use prime_factors::factors;

#[test]
fn test_no_factors() {
    assert_eq!(factors(1), vec![]);
}

#[test]
fn test_prime_number() {
    assert_eq!(factors(2), vec![2]);
}

#[test]
fn test_square_of_a_prime() {
    assert_eq!(factors(9), vec![3, 3]);
}

#[test]
fn test_cube_of_a_prime() {
    assert_eq!(factors(8), vec![2, 2, 2]);
}

#[test]
fn test_product_of_primes_and_non_primes() {
    assert_eq!(factors(12), vec![2, 2, 3]);
}

#[test]
fn test_product_of_primes() {
    assert_eq!(factors(901_255), vec![5, 17, 23, 461]);
}

#[test]
fn test_factors_include_large_prime() {
    let first_time = SystemTime::now();
    println!("first time: {:?}", first_time);

    assert_eq!(factors(93_819_012_551), vec![11, 9539, 894_119]);

    let second_time = SystemTime::now();
    let first_duration = second_time.duration_since(first_time).unwrap();
    println!(
        "second time: {:?} -- first_duration: {:?}",
        second_time, first_duration
    );

    assert_eq!(factors(93_819_012_551), vec![11, 9539, 894_119]);

    let third_time = SystemTime::now();
    let second_duration = third_time.duration_since(second_time).unwrap();
    println!(
        "third time: {:?} -- second_duration: {:?}",
        third_time, second_duration
    );

    assert!(first_duration > second_duration * 3);
    assert!(first_duration >= Duration::from_secs(60));
    assert!(second_duration <= Duration::from_secs(5));
}
