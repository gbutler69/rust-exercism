use std::{cmp::Ordering, collections::HashSet};

struct Hand<'a> {
    cards: &'a str,
    hand_type: HandType,
}

enum HandType {
    StraightFlush {
        rank: u8,
    },
    FourOfKind {
        rank: u8,
        other: u8,
    },
    FullHouse {
        rank1: u8,
        rank2: u8,
    },
    Flush {
        rank1: u8,
        rank2: u8,
        rank3: u8,
        rank4: u8,
        rank5: u8,
    },
    Straight {
        rank: u8,
    },
    ThreeOfKind {
        rank: u8,
        other1: u8,
        other2: u8,
    },
    TwoPair {
        rank1: u8,
        rank2: u8,
        other: u8,
    },
    Pair {
        rank: u8,
        other1: u8,
        other2: u8,
        other3: u8,
    },
    Nothing {
        rank1: u8,
        rank2: u8,
        rank3: u8,
        rank4: u8,
        rank5: u8,
    },
}

impl HandType {
    fn cmp_1_rank(a1: &u8, a2: &u8) -> Ordering {
        a1.cmp(&a2)
    }
    fn cmp_2_ranks(a1: &u8, a2: &u8, b1: &u8, b2: &u8) -> Ordering {
        let cmp = a1.cmp(&a2);
        if let Ordering::Equal = cmp {
            b1.cmp(&b2)
        } else {
            cmp
        }
    }
    fn cmp_3_ranks(a1: &u8, a2: &u8, b1: &u8, b2: &u8, c1: &u8, c2: &u8) -> Ordering {
        match a1.cmp(&a2) {
            Ordering::Equal => match b1.cmp(&b2) {
                Ordering::Equal => c1.cmp(&c2),
                order => order,
            },
            order => order,
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn cmp_4_ranks(
        a1: &u8,
        a2: &u8,
        b1: &u8,
        b2: &u8,
        c1: &u8,
        c2: &u8,
        d1: &u8,
        d2: &u8,
    ) -> Ordering {
        match a1.cmp(&a2) {
            Ordering::Equal => match b1.cmp(&b2) {
                Ordering::Equal => match c1.cmp(&c2) {
                    Ordering::Equal => d1.cmp(&d2),
                    order => order,
                },
                order => order,
            },
            order => order,
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn cmp_5_ranks(
        a1: &u8,
        a2: &u8,
        b1: &u8,
        b2: &u8,
        c1: &u8,
        c2: &u8,
        d1: &u8,
        d2: &u8,
        e1: &u8,
        e2: &u8,
    ) -> Ordering {
        match a1.cmp(&a2) {
            Ordering::Equal => match b1.cmp(&b2) {
                Ordering::Equal => match c1.cmp(&c2) {
                    Ordering::Equal => match d1.cmp(&d2) {
                        Ordering::Equal => e1.cmp(&e2),
                        order => order,
                    },
                    order => order,
                },
                order => order,
            },
            order => order,
        }
    }
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for HandType {}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            // --
            (HandType::StraightFlush { rank: ra }, // --
                HandType::StraightFlush { rank: rb } ) // --
                => Self::cmp_1_rank(ra,rb),
            (HandType::StraightFlush { .. }, _) => Ordering::Greater,
            // --
            (HandType::FourOfKind { .. }, HandType::StraightFlush { .. }) => Ordering::Less,
            (HandType::FourOfKind { rank: ra, other: ha }, // --
                HandType::FourOfKind { rank: rb, other: hb }) // --
                => Self::cmp_2_ranks( ra, rb, ha, hb ),
            (HandType::FourOfKind { .. }, _) => Ordering::Greater,
            // --
            (HandType::FullHouse { .. }, HandType::StraightFlush { .. }) => Ordering::Less,
            (HandType::FullHouse { .. }, HandType::FourOfKind { .. }) => Ordering::Less,
            (HandType::FullHouse { rank1: r1a, rank2: r2a }, // --
                HandType::FullHouse { rank1: r1b, rank2: r2b }) // --
                => Self::cmp_2_ranks(r1a, r1b, r2a, r2b),
            (HandType::FullHouse { .. }, _) => Ordering::Greater,
            // --
            (HandType::Flush { .. }, HandType::StraightFlush { .. }) => Ordering::Less,
            (HandType::Flush { .. }, HandType::FourOfKind { .. }) => Ordering::Less,
            (HandType::Flush { .. }, HandType::FullHouse { .. }) => Ordering::Less,
            (HandType::Flush { rank1: r1a, rank2: r2a, rank3: r3a, rank4: r4a, rank5: r5a }, // --
                HandType::Flush { rank1: r1b, rank2: r2b, rank3: r3b, rank4: r4b, rank5: r5b }) // --
                => Self::cmp_5_ranks( r1a, r1b, r2a, r2b, r3a, r3b, r4a, r4b, r5a, r5b ),
            (HandType::Flush { .. }, _) => Ordering::Greater,
            // --
            (HandType::Straight { rank: ra }, // --
                HandType::Straight { rank: rb }) // --
                => Self::cmp_1_rank(ra,rb),
            (HandType::Straight { .. }, HandType::ThreeOfKind { .. }) => Ordering::Greater,
            (HandType::Straight { .. }, HandType::TwoPair { .. }) => Ordering::Greater,
            (HandType::Straight { .. }, HandType::Pair { .. }) => Ordering::Greater,
            (HandType::Straight { .. }, HandType::Nothing { .. }) => Ordering::Greater,
            (HandType::Straight { .. }, _) => Ordering::Less,
            // --
            (HandType::ThreeOfKind { rank: ra, other1: o1a, other2: o2a }, // --
                HandType::ThreeOfKind { rank: rb, other1: o1b, other2: o2b }) // --
                => Self::cmp_3_ranks(ra,rb,o1a,o1b,o2a,o2b),
            (HandType::ThreeOfKind { .. }, HandType::TwoPair { .. }) => Ordering::Greater,
            (HandType::ThreeOfKind { .. }, HandType::Pair { .. }) => Ordering::Greater,
            (HandType::ThreeOfKind { .. }, HandType::Nothing { .. }) => Ordering::Greater,
            (HandType::ThreeOfKind { .. }, _) => Ordering::Less,
            // --
            (HandType::TwoPair { rank1: r1a, rank2: r2a, other: oa }, // --
                HandType::TwoPair { rank1: r1b, rank2: r2b, other: ob }) // --
                => Self::cmp_3_ranks(r1a, r1b, r2a, r2b, oa, ob),
            (HandType::TwoPair { .. }, HandType::Pair { .. }) => Ordering::Greater,
            (HandType::TwoPair { .. }, HandType::Nothing { .. }) => Ordering::Greater,
            (HandType::TwoPair { .. }, _) => Ordering::Less,
            // --
            (HandType::Pair { rank: ra, other1: o1a, other2: o2a, other3: o3a }, // --
                HandType::Pair { rank: rb, other1: o1b, other2: o2b, other3: o3b }) // --
                => Self::cmp_4_ranks(ra,rb,o1a,o1b,o2a,o2b,o3a,o3b),
            (HandType::Pair { .. }, HandType::Nothing { .. }) => Ordering::Greater,
            (HandType::Pair { .. }, _) => Ordering::Less,
            // --
            (HandType::Nothing { rank1: r1a, rank2: r2a, rank3: r3a, rank4: r4a, rank5: r5a }, // --
                HandType::Nothing { rank1: r1b, rank2: r2b, rank3: r3b, rank4: r4b, rank5: r5b }) // --
                => Self::cmp_5_ranks( r1a, r1b, r2a, r2b, r3a, r3b, r4a, r4b, r5a, r5b ),
            (HandType::Nothing { .. }, _) => Ordering::Less,
        }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    winning(hands.iter().copied().map(to_hand).collect::<Vec<_>>())
}

fn to_hand(cards: &str) -> Hand<'_> {
    Hand {
        cards,
        hand_type: to_hand_type(cards),
    }
}

fn to_hand_type(cards: &str) -> HandType {
    let (ranks, suits) = sorted_ranks_and_num_suits_in(cards);
    is_straight_flush_from(&ranks, suits)
        .or_else(|| is_4_of_kind_from(&ranks))
        .or_else(|| is_full_house_from(&ranks))
        .or_else(|| is_flush_from(&ranks, suits))
        .or_else(|| is_straight_from(&ranks))
        .or_else(|| is_3_of_kind_from(&ranks))
        .or_else(|| is_2_pair_from(&ranks))
        .or_else(|| is_pair_from(&ranks))
        .or_else(|| is_nothing_from(&ranks))
        .unwrap()
}

fn sorted_ranks_and_num_suits_in(cards: &str) -> (Vec<u8>, u8) {
    let mut ranks = Vec::new();
    let mut suits = HashSet::new();
    for card in cards.split_whitespace() {
        match card.find(char::is_alphabetic) {
            Some(first_alpha_at) if first_alpha_at > 0 => {
                let (rank, suit) = card.split_at(first_alpha_at);
                let rank = str::parse::<u8>(rank).unwrap();
                ranks.push(rank);
                suits.insert(suit);
            }
            _ => {
                ranks.push(match &card[0..=0] {
                    "J" => 11,
                    "Q" => 12,
                    "K" => 13,
                    "A" => 14,
                    _ => panic!("invalid rank"),
                });
                suits.insert(&card[1..=1]);
            }
        }
    }
    ranks.sort_unstable();
    ranks.reverse();
    (ranks, suits.len() as u8)
}

/// assumes sorted input
fn number_of_kind_rank_and_remaining(group_size: u8, ranks: &[u8]) -> (Option<u8>, Vec<u8>) {
    let mut matched_rank = 0;
    let mut remaining = Vec::new();
    for (idx, ranks_window) in ranks.windows(group_size as usize).enumerate() {
        let first_of_window = ranks_window[0];
        if ranks_window.iter().skip(1).all(|r| *r == first_of_window) {
            matched_rank = first_of_window;
            ranks
                .iter()
                .take(idx)
                .chain(ranks.iter().skip(idx + group_size as usize))
                .for_each(|r| remaining.push(*r));
            break;
        }
    }
    if matched_rank >= 2 {
        (Some(matched_rank), remaining)
    } else {
        (None, remaining)
    }
}

fn is_straight_flush_from(ranks: &[u8], suits: u8) -> Option<HandType> {
    match (
        suits,
        ranks.windows(2).all(|window| window[0] - 1 == window[1]),
    ) {
        (1, true) => Some(HandType::StraightFlush { rank: ranks[4] }),
        (1, false) => {
            let mut ranks = ranks
                .iter()
                .copied()
                .map(|v| if v == 14 { 1 } else { v })
                .collect::<Vec<_>>();
            ranks.sort_unstable();
            ranks.reverse();
            match ranks.windows(2).all(|window| window[0] - 1 == window[1]) {
                true => Some(HandType::StraightFlush { rank: ranks[4] }),
                _ => None,
            }
        }
        _ => None,
    }
}
fn is_4_of_kind_from(ranks: &[u8]) -> Option<HandType> {
    match number_of_kind_rank_and_remaining(4, ranks) {
        (Some(matched_rank), others) => Some(HandType::FourOfKind {
            rank: matched_rank,
            other: others[0],
        }),
        _ => None,
    }
}
fn is_full_house_from(ranks: &[u8]) -> Option<HandType> {
    match number_of_kind_rank_and_remaining(3, ranks) {
        (Some(first_matched_rank), others) => match number_of_kind_rank_and_remaining(2, &others) {
            (Some(second_matched_rank), _) => Some(HandType::FullHouse {
                rank1: first_matched_rank,
                rank2: second_matched_rank,
            }),
            _ => None,
        },
        _ => None,
    }
}
fn is_flush_from(ranks: &[u8], suits: u8) -> Option<HandType> {
    match suits {
        1 => Some(HandType::Flush {
            rank1: ranks[0],
            rank2: ranks[1],
            rank3: ranks[2],
            rank4: ranks[3],
            rank5: ranks[4],
        }),
        _ => None,
    }
}
fn is_straight_from(ranks: &[u8]) -> Option<HandType> {
    match ranks.windows(2).all(|window| window[0] - 1 == window[1]) {
        true => Some(HandType::Straight { rank: ranks[4] }),
        false => {
            let mut ranks = ranks
                .iter()
                .copied()
                .map(|v| if v == 14 { 1 } else { v })
                .collect::<Vec<_>>();
            ranks.sort_unstable();
            ranks.reverse();
            match ranks.windows(2).all(|window| window[0] - 1 == window[1]) {
                true => Some(HandType::Straight { rank: ranks[4] }),
                _ => None,
            }
        }
    }
}
fn is_3_of_kind_from(ranks: &[u8]) -> Option<HandType> {
    match number_of_kind_rank_and_remaining(3, ranks) {
        (Some(matched_rank), others) => Some(HandType::ThreeOfKind {
            rank: matched_rank,
            other1: others[0],
            other2: others[1],
        }),
        _ => None,
    }
}
fn is_2_pair_from(ranks: &[u8]) -> Option<HandType> {
    match number_of_kind_rank_and_remaining(2, ranks) {
        (Some(first_matched_rank), others) => match number_of_kind_rank_and_remaining(2, &others) {
            (Some(second_matched_rank), others) => Some(HandType::TwoPair {
                rank1: first_matched_rank,
                rank2: second_matched_rank,
                other: others[0],
            }),
            _ => None,
        },
        _ => None,
    }
}
fn is_pair_from(ranks: &[u8]) -> Option<HandType> {
    match number_of_kind_rank_and_remaining(2, ranks) {
        (Some(matched_rank), others) => Some(HandType::Pair {
            rank: matched_rank,
            other1: others[0],
            other2: others[1],
            other3: others[2],
        }),
        _ => None,
    }
}
fn is_nothing_from(ranks: &[u8]) -> Option<HandType> {
    Some(HandType::Nothing {
        rank1: ranks[0],
        rank2: ranks[1],
        rank3: ranks[2],
        rank4: ranks[3],
        rank5: ranks[4],
    })
}

fn winning(mut hands: Vec<Hand<'_>>) -> Option<Vec<&str>> {
    hands.sort_by(|a, b| a.hand_type.cmp(&b.hand_type));
    hands.reverse();
    let mut hands = hands.into_iter();
    if let Some(mut hand) = hands.next() {
        let mut winning_hands = Vec::new();
        loop {
            winning_hands.push(hand.cards);
            match hands.next() {
                Some(next_hand) if next_hand.hand_type == hand.hand_type => hand = next_hand,
                _ => break,
            }
        }
        Some(winning_hands)
    } else {
        None
    }
}
