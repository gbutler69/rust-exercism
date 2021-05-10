#[derive(Copy, Clone, Default)]
struct Group {
    pub book1: bool,
    pub book2: bool,
    pub book3: bool,
    pub book4: bool,
    pub book5: bool,
}

impl Group {
    pub fn add(&mut self, book: u8) {
        match book {
            1 => self.set_book_1_or_panic(),
            2 => self.set_book_2_or_panic(),
            3 => self.set_book_3_or_panic(),
            4 => self.set_book_4_or_panic(),
            5 => self.set_book_5_or_panic(),
            _ => panic!("only book 1 through 5 supported"),
        }
    }
    fn set_book_1_or_panic(&mut self) {
        if self.book1 {
            panic!("Unable to set book 1 in this group when already set")
        }
        self.book1 = true;
    }
    fn set_book_2_or_panic(&mut self) {
        if self.book2 {
            panic!("Unable to set book 2 in this group when already set")
        }
        self.book2 = true;
    }
    fn set_book_3_or_panic(&mut self) {
        if self.book3 {
            panic!("Unable to set book 3 in this group when already set")
        }
        self.book3 = true;
    }
    fn set_book_4_or_panic(&mut self) {
        if self.book4 {
            panic!("Unable to set book 4 in this group when already set")
        }
        self.book4 = true;
    }
    fn set_book_5_or_panic(&mut self) {
        if self.book5 {
            panic!("Unable to set book 5 in this group when already set")
        }
        self.book5 = true;
    }
    #[allow(clippy::identity_op)]
    pub fn len(&self) -> usize {
        0_usize
            + if self.book1 { 1_usize } else { 0_usize }
            + if self.book2 { 1_usize } else { 0_usize }
            + if self.book3 { 1_usize } else { 0_usize }
            + if self.book4 { 1_usize } else { 0_usize }
            + if self.book5 { 1_usize } else { 0_usize }
    }
    pub fn cost(&self) -> u32 {
        match self.len() {
            0 => 0,
            1 => 8 * 100,
            2 => 8 * 95 * 2,
            3 => 8 * 90 * 3,
            4 => 8 * 80 * 4,
            _ => 8 * 75 * 5,
        }
    }
    pub fn possibly_donate_to(&mut self, other: &mut Self) -> bool {
        if self.len() >= other.len() + 2 {
            if self.book1 && !other.book1 {
                self.book1 = false;
                other.book1 = true;
                true
            } else if self.book2 && !other.book2 {
                self.book2 = false;
                other.book2 = true;
                true
            } else if self.book3 && !other.book3 {
                self.book3 = false;
                other.book3 = true;
                true
            } else if self.book4 && !other.book4 {
                self.book4 = false;
                other.book4 = true;
                true
            } else if self.book5 && !other.book5 {
                self.book5 = false;
                other.book5 = true;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
pub fn lowest_price(books: &[u32]) -> u32 {
    let mut groups = vec![Group::default(); books.len()];
    for i in 1..=5 {
        books
            .iter()
            .copied()
            .map(|b| b as u8)
            .filter(|num| *num == i)
            .zip(groups.iter_mut())
            .for_each(|(book, group)| group.add(book))
    }
    balance_book_groups(groups)
        .iter()
        .map(|group| group.cost())
        .sum::<u32>()
}

fn balance_book_groups(mut groups: Vec<Group>) -> Vec<Group> {
    let first_3_at = groups.iter().position(|group| group.len() == 3);
    if let Some(pos) = first_3_at {
        let (four_or_more, three_or_less) = groups.split_at_mut(pos);
        let first_4_at = four_or_more.iter().position(|group| group.len() == 4);
        let (fives, _) = match first_4_at {
            Some(pos) => four_or_more.split_at_mut(pos),
            None => four_or_more.split_at_mut(four_or_more.len()),
        };
        for five in fives {
            for three in three_or_less.iter_mut().filter(|group| group.len() == 3) {
                if five.len() == 5 && five.possibly_donate_to(three) {
                    break;
                }
            }
        }
    }
    groups
}
