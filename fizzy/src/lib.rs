/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<'a, T: Copy + ToString> {
    match_func: Box<dyn Fn(T) -> bool + 'a>,
    sub: String,
}

impl<'a, T: Copy + ToString> Matcher<'a, T> {
    pub fn new<MatcherFunc: Fn(T) -> bool + 'a, S: ToString + 'a>(
        matcher: MatcherFunc,
        subs: S,
    ) -> Self {
        Self {
            match_func: Box::new(matcher),
            sub: subs.to_string(),
        }
    }
    pub fn apply(&self, element: T) -> Option<String> {
        match (*self.match_func)(element) {
            true => Some(self.sub.to_string()),
            false => None,
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
pub struct Fizzy<'a, T: Copy + ToString> {
    matchers: Vec<Matcher<'a, T>>,
}

impl<'a, T: Copy + ToString + 'a> Fizzy<'a, T> {
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    pub fn add_matcher(mut self, matcher: Matcher<'a, T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply(self, iter: impl Iterator<Item = T> + 'a) -> impl Iterator<Item = String> + 'a {
        iter.map(move |element| self.apply_to_element(element))
    }

    fn apply_to_element(&self, element: T) -> String {
        let mut result = String::with_capacity(5 * self.matchers.len());
        for matcher in self.matchers.iter() {
            if let Some(output) = matcher.apply(element) {
                result += output.as_str();
            }
        }
        match result.len() {
            0 => element.to_string(),
            _ => result,
        }
    }
}

impl<'a, T: Copy + ToString + 'a> Default for Fizzy<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<'a, T: 'a + Copy + std::ops::Rem<Output = T> + From<u8> + PartialEq + ToString>(
) -> Fizzy<'a, T> {
    let fizzy = Fizzy::default();
    fizzy
        .add_matcher(Matcher::new(
            |element| element % T::from(3) == T::from(0),
            "fizz",
        ))
        .add_matcher(Matcher::new(
            |element| element % T::from(5) == T::from(0),
            "buzz",
        ))
}
