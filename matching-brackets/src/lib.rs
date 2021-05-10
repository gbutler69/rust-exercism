pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    let mut mismatched_end_brace_found = false;
    for c in string.chars() {
        match c {
            '{' | '(' | '[' => stack.push(c),
            '}' if !top_of(&mut stack, '{') => mismatched_end_brace_found = true,
            ')' if !top_of(&mut stack, '(') => mismatched_end_brace_found = true,
            ']' if !top_of(&mut stack, '[') => mismatched_end_brace_found = true,
            _ => (),
        }
        if mismatched_end_brace_found {
            break;
        }
    }
    !mismatched_end_brace_found && stack.is_empty()
}

fn top_of(stack: &mut Vec<char>, is_char: char) -> bool {
    match stack.pop() {
        Some(top) if top == is_char => true,
        _ => false,
    }
}
