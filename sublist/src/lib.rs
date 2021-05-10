#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.is_sublist_of(second_list) {
        if first_list.len() == second_list.len() {
            Comparison::Equal
        } else {
            Comparison::Sublist
        }
    } else if second_list.is_sublist_of(first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

trait Sublist<T: PartialEq> {
    fn is_sublist_of(self, other: Self) -> bool;
    fn is_a_sublist_of(first: &[T], second: &[T]) -> bool {
        if first.len() == 0 {
            return true;
        } else if second.len() == 0 {
            return false;
        }
        let (mut begins_at, mut ends_at) = (0_usize, second.len() - 1);
        while ends_at - begins_at >= first.len()
            && (second[begins_at] != first[0] || second[ends_at] != first[first.len() - 1])
        {
            if second[begins_at] != first[0] && begins_at < second.len() - first.len() {
                begins_at += 1
            } else {
                ends_at = begins_at + first.len() - 1;
            }
            if second[ends_at] != first[first.len() - 1] && ends_at > first.len() {
                ends_at -= 1
            } else {
                begins_at = ends_at + 1 - first.len();
            }
        }
        if ends_at - begins_at + 1 < first.len() {
            return false;
        }
        while ends_at > first.len() || begins_at < second.len() + 1 - first.len() {
            if first
                .iter()
                .zip(second[begins_at..(begins_at + first.len())].iter())
                .all(|(a, b)| a == b)
            {
                return true;
            }
            if first
                .iter()
                .zip(second[(ends_at + 1 - first.len())..=ends_at].iter())
                .all(|(a, b)| a == b)
            {
                return true;
            }
            begins_at += 1;
            if ends_at > first.len() {
                ends_at -= 1
            };
        }
        if ends_at < begins_at || ends_at - begins_at < first.len() {
            return false;
        }
        first
            .iter()
            .zip(second[begins_at..=ends_at].iter())
            .all(|(a, b)| a == b)
    }
}

impl<T: PartialEq> Sublist<T> for &[T] {
    fn is_sublist_of(self, other: Self) -> bool {
        Self::is_a_sublist_of(self, other)
    }
}
