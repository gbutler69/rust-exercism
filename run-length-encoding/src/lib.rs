struct EncodingState {
    cur: char,
    count: u32,
}

pub fn encode(source: &str) -> String {
    if source.trim().is_empty() {
        return "".into();
    }

    let mut state = EncodingState {
        cur: '\0',
        count: 0,
    };

    source
        .chars()
        .chain(std::iter::once('\0'))
        .filter_map(|cur| compress(&mut state, cur))
        .flatten()
        .collect()
}

fn compress<'a>(
    state: &'a mut EncodingState,
    current: char,
) -> Option<impl Iterator<Item = String>> {
    match *state {
        EncodingState { cur, .. } if cur == current => {
            state.count += 1;
            None
        }
        EncodingState { cur, count } if count != 0 => {
            let rv = match count {
                1 => cur.to_string(),
                _ => format!("{}{}", count, cur),
            };
            state.count = 1;
            state.cur = current;
            Some(std::iter::once(rv))
        }
        EncodingState { .. } => {
            state.count = 1;
            state.cur = current;
            None
        }
    }
}

pub fn decode(source: &str) -> String {
    let mut repetitions_state = 0_u32;
    source
        .chars()
        .filter_map(|cur| uncompress(&mut repetitions_state, cur))
        .flatten()
        .collect()
}

fn uncompress(repetitions: &mut u32, current: char) -> Option<impl Iterator<Item = String>> {
    match (current, *repetitions) {
        ('1'..='9', 0) => {
            *repetitions = current as u32 - '0' as u32;
            None
        }
        ('0'..='9', _) => {
            *repetitions = *repetitions * 10 + current as u32 - '0' as u32;
            None
        }
        (_, 0) => {
            let rv = current.to_string().repeat(1);
            *repetitions = 0;
            Some(std::iter::once(rv))
        }
        _ => {
            let rv = current.to_string().repeat(*repetitions as usize);
            *repetitions = 0;
            Some(std::iter::once(rv))
        }
    }
}
