pub fn is_armstrong_number(num: u32) -> bool {
    if num <= 9 {
        true
    } else {
        let mut n = num;
        let total_digits = (n as f64).log10().floor() as u32 + 1;
        let mut digits_sum = 0;
        while n > 0 {
            let digit = n % 10;
            n /= 10;
            digits_sum += digit.pow(total_digits)
        }
        digits_sum == num
    }
}
