use nth_prime as np;

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]
fn test_10th_prime() {
    assert_eq!(np::nth(9), 29);
}

#[test]
fn test_11th_prime() {
    assert_eq!(np::nth(10), 31);
}

#[test]
fn test_12th_prime() {
    assert_eq!(np::nth(11), 37);
}

#[test]
fn test_big_prime() {
    assert_eq!(np::nth(10_000), 104_743);
}
