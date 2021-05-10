#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if let Err(e) = validate_bases(from_base, to_base) {
        return Err(e);
    }
    match digits_to_value(number, from_base) {
        Ok(0) => Ok(vec![0]),
        Err(e) => Err(e),
        Ok(from_value) => value_to_digits(from_value, to_base),
    }
}

fn validate_bases(from_base: u32, to_base: u32) -> Result<(), Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    Ok(())
}

fn digits_to_value(number: &[u32], from_base: u32) -> Result<u32, Error> {
    (0..(number.len() as u32))
        .rev()
        .zip(number.iter())
        .map(|(place, digit)| to_value_for_place(digit, from_base, place))
        .sum::<Result<u32, _>>()
}

fn to_value_for_place(digit: &u32, from_base: u32, place: u32) -> Result<u32, Error> {
    if *digit >= from_base {
        Err(Error::InvalidDigit(*digit))
    } else {
        Ok(digit * from_base.pow(place))
    }
}

fn value_to_digits(from_value: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    let mut result = (0..)
        .scan(from_value, |from_value, _| {
            to_digit_for_place(from_value, to_base)
        })
        .collect::<Vec<u32>>();
    result.reverse();
    Ok(result)
}

fn to_digit_for_place(from_value: &mut u32, to_base: u32) -> Option<u32> {
    if *from_value == 0 {
        return None;
    }
    let digit_for_place = *from_value % to_base;
    *from_value /= to_base;
    Some(digit_for_place)
}
