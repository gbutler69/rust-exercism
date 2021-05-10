pub fn find<T, A: AsRef<[T]>>(array: A, key: T) -> Option<usize>
where
    T: Eq + Ord,
{
    let array = array.as_ref();
    find_in_slice(array, key)
}

fn find_in_slice<T>(array: &[T], key: T) -> Option<usize>
where
    T: Eq + Ord,
{
    let len = array.len();
    match len {
        0 => None,
        1 if array[0] != key => None,
        1 if array[0] == key => Some(0),
        _ => check_mid_matches_or_split_and_find(array, key),
    }
}

fn check_mid_matches_or_split_and_find<T>(array: &[T], key: T) -> Option<usize>
where
    T: Eq + Ord,
{
    let mid = array.len() / 2;
    match &array[mid] {
        el_val if key == *el_val => Some(mid),
        v => find_in_splits(array, mid, key, v),
    }
}

fn find_in_splits<T>(array: &[T], mid: usize, key: T, v: &T) -> Option<usize>
where
    T: Eq + Ord,
{
    let (left, right) = array.split_at(mid);
    match key > *v {
        true => find(right, key).map(|v| v + mid),
        false => find(left, key),
    }
}
