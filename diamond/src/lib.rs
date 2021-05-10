pub fn get_diamond(c: char) -> Vec<String> {
    let mut result = Vec::new();
    if !('A'..='Z').contains(&c) {
        return result;
    }
    let max_fill = c as usize - 'A' as usize;
    let width = max_fill * 2 + 1;
    for fill in (0..=max_fill).rev().chain(1..=max_fill) {
        let mut line = vec![b' '; width];
        line[fill] = c as u8 - fill as u8;
        line[width - fill - 1] = c as u8 - fill as u8;
        // SAFETY/SOUNDNESS: This is Safe/Sound because the above
        // logic guarantees that ONLY the u8 values for the characters
        // A through Z (ASCII) will exist in the line vec. This is
        // guaranteed to be valid UTF-8 as ASCII is valid UTF-8
        result.push(unsafe { String::from_utf8_unchecked(line) });
    }
    result
}
