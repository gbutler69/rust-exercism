/// What should the type of _function be?
pub fn map<T, U, Func: FnMut(T) -> U>(input: Vec<T>, mut function: Func) -> Vec<U> {
    let mut result = Vec::with_capacity(input.len());
    for val in input {
        result.push(function(val));
    }
    result
}
