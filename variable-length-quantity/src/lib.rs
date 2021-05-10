#![feature(custom_inner_attributes)]
#![feature(array_map)]

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.into_iter().flat_map(vlq_encode).collect()
}

fn vlq_encode(v: &u32) -> impl Iterator<Item = u8> {
    #![rustfmt::skip]
    let seven_bit_groups = [
        ((*v) >> 28) as u8 | 0x80,
        ((*v) >> 21) as u8 | 0x80,
        ((*v) >> 14) as u8 | 0x80,
        ((*v) >>  7) as u8 | 0x80,
        ( *v)        as u8 & 0x7F,
    ];
    let mut group_num = 0;
    let mut leading_zeroes_processed = false;
    std::iter::from_fn(move || -> Option<u8> {
        while group_num <= 4 {
            let byte = seven_bit_groups[group_num as usize];
            group_num += 1;
            if let false = leading_zeroes_processed {if byte == 0x80 {continue}}
            leading_zeroes_processed = true;
            return Some(byte)
        }
        None
    })
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut rv = Vec::new();
    let mut accum = 0_u64;
    let mut end_sequence_seen = false;
    for byte in bytes {
        end_sequence_seen = false;
        accum = (accum << 7) | (*byte & 0x7F_u8) as u64;
        if accum > u32::MAX as u64 {
            return Err(Error::Overflow);
        }
        if *byte & 0x80_u8 == 0 {
            rv.push(accum as u32);
            accum = 0;
            end_sequence_seen = true;
        }
    }
    if let false = end_sequence_seen {
        return Err(Error::IncompleteNumber);
    }
    Ok(rv)
}
