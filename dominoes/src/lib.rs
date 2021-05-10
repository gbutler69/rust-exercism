use std::collections::HashMap;

use rayon::prelude::*;

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        Some(Vec::new())
    } else if input.len() == 1 {
        if input[0].0 == input[0].1 {
            Some(vec![(input[0].0, input[0].1)])
        } else {
            None
        }
    } else {
        StartChain::from(BoneYard::from(input)).maybe_into_valid_chain()
    }
}

#[derive(Clone)]
struct BoneYard {
    bones: Vec<(u8, u8)>,
    left_index: HashMap<u8, Vec<usize>>,
    right_index: HashMap<u8, Vec<usize>>,
}

impl BoneYard {
    fn from(bones: &[(u8, u8)]) -> Self {
        let mut left_index = HashMap::new();
        let mut right_index = HashMap::new();
        for (idx, bone) in bones.iter().enumerate() {
            left_index.entry(bone.0).or_insert_with(Vec::new).push(idx);
            right_index.entry(bone.1).or_insert_with(Vec::new).push(idx);
        }
        Self {
            bones: bones.to_vec(),
            left_index,
            right_index,
        }
    }

    fn size(&self) -> usize {
        self.bones.len()
    }

    #[allow(clippy::type_complexity)]
    fn start_pairs(self) -> Vec<(BoneYard, (u8, u8), (u8, u8))> {
        let mut starting_pairs = Vec::new();
        for (i, start_bone) in self.bones.iter().enumerate() {
            for (j, end_bone) in self.bones.iter().enumerate().skip(i + 1) {
                self.if_match_then_use_pair(*start_bone, *end_bone, i, j, &mut starting_pairs);
                self.if_match_then_use_pair(
                    (start_bone.1, start_bone.0),
                    *end_bone,
                    i,
                    j,
                    &mut starting_pairs,
                );
                self.if_match_then_use_pair(
                    *start_bone,
                    (end_bone.1, end_bone.0),
                    i,
                    j,
                    &mut starting_pairs,
                );
                self.if_match_then_use_pair(
                    (start_bone.1, start_bone.0),
                    (end_bone.1, end_bone.0),
                    i,
                    j,
                    &mut starting_pairs,
                );
            }
        }
        starting_pairs
    }

    #[allow(clippy::type_complexity)]
    fn if_match_then_use_pair(
        &self,
        start: (u8, u8),
        end: (u8, u8),
        start_idx: usize,
        end_idx: usize,
        pairs: &mut Vec<(BoneYard, (u8, u8), (u8, u8))>,
    ) {
        if start.0 == end.1 {
            let mut boneyard = (*self).clone();
            boneyard.remove(end_idx);
            boneyard.remove(start_idx);
            pairs.push((boneyard, start, end));
        }
    }

    fn remove(&mut self, idx: usize) {
        let removed_bone = self.bones.remove(idx);
        self.left_index.entry(removed_bone.0).and_modify(|v| {
            if let Some(i) = v.iter().position(|v| *v == idx) {
                v.remove(i);
            }
        });
        self.right_index.entry(removed_bone.1).and_modify(|v| {
            if let Some(i) = v.iter().position(|v| *v == idx) {
                v.remove(i);
            }
        });
    }

    fn if_last_fits_then_use(self, front: u8, back: u8) -> Option<(u8, u8)> {
        if !self.bones.is_empty() {
            let last = self.bones[0];
            if last.0 == front && last.1 == back {
                Some(last)
            } else if last.1 == front && last.0 == back {
                Some((last.1, last.0))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn next_matches(self, front: u8) -> Vec<(BoneYard, (u8, u8))> {
        self.bones
            .iter()
            .enumerate()
            .flat_map(|(idx, link)| {
                if link.0 == front {
                    Some((self.take_remaining(idx), *link))
                } else if link.1 == front {
                    Some((self.take_remaining(idx), (link.1, link.0)))
                } else {
                    None
                }
            })
            .collect()
    }

    fn take_remaining(&self, idx: usize) -> BoneYard {
        let mut bone_yard = self.clone();
        bone_yard.remove(idx);
        bone_yard
    }
}

struct StartChain {
    boneyard: BoneYard,
}

struct PartialChain {
    chain: Vec<(u8, u8)>,
    boneyard: BoneYard,
    next_link: usize,
}

struct CompletedChain {
    chain: Vec<(u8, u8)>,
}

impl StartChain {
    fn from(boneyard: BoneYard) -> Self {
        Self { boneyard }
    }

    fn maybe_into_valid_chain(self) -> Option<Vec<(u8, u8)>> {
        let starting_link_sets = self.starting_link_sets();
        starting_link_sets
            .into_par_iter()
            .map(PartialChain::maybe_to_completed_chain)
            .find_first(Option::is_some)
            .map(Option::unwrap)
            .map(CompletedChain::into_vec)
    }

    fn starting_link_sets(self) -> Vec<PartialChain> {
        self.boneyard
            .start_pairs()
            .into_iter()
            .map(PartialChain::from)
            .collect()
    }
}

impl PartialChain {
    fn from((boneyard, beginning, end): (BoneYard, (u8, u8), (u8, u8))) -> Self {
        let mut chain = vec![Default::default(); boneyard.size() + 2];
        chain[0] = beginning;
        chain[boneyard.size() + 1] = end;
        Self {
            chain,
            boneyard,
            next_link: 1,
        }
    }
    fn maybe_to_completed_chain(self) -> Option<CompletedChain> {
        if self.boneyard.size() == 0 {
            if self.chain[self.next_link - 1] == self.chain[self.chain.len() - 1] {
                Some(CompletedChain { chain: self.chain })
            } else {
                None
            }
        } else if self.boneyard.size() == 1 {
            match self.boneyard.if_last_fits_then_use(
                self.chain[self.next_link - 1].1,
                self.chain[self.chain.len() - 1].0,
            ) {
                Some(last_link) => {
                    let mut completed = CompletedChain { chain: self.chain };
                    completed.chain[self.next_link] = last_link;
                    Some(completed)
                }
                None => None,
            }
        } else {
            let (boneyard, chain, next_link) = (self.boneyard, self.chain, self.next_link);
            boneyard
                .next_matches(chain[self.next_link - 1].1)
                .into_par_iter()
                .map(|(boneyard, link)| {
                    let mut chain = chain.clone();
                    chain[next_link] = link;
                    PartialChain {
                        chain,
                        boneyard,
                        next_link: next_link + 1,
                    }
                })
                .map(|pc| pc.maybe_to_completed_chain())
                .find_first(Option::is_some)
                .map(Option::unwrap)
        }
    }
}

impl CompletedChain {
    fn into_vec(self) -> Vec<(u8, u8)> {
        self.chain
    }
}
