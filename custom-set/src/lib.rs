#![feature(type_alias_impl_trait)]

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Debug)]
pub struct CustomSet<T: Copy + Eq + Hash> {
    storage: Vec<Vec<T>>,
}

impl<T: Copy + Eq + Hash> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.is_subset(other) && other.is_subset(self)
    }
}

impl<T: Copy + Eq + Hash> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut this = Self {
            storage: vec![Vec::new(); (input.len() * 2).max(4)],
        };
        for &t in input {
            this.add(t);
        }
        this
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            storage: vec![Vec::new(); (capacity * 2).max(4)],
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.contained_in_bucket(self.bucket_for(*element), element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contained_in_bucket(self.bucket_for(element), &element) {
            self.add_to_bucket(self.bucket_for(element), element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.into_iter().all(|element| other.contains(&element))
    }

    pub fn is_empty(&self) -> bool {
        self.storage.iter().all(Vec::is_empty)
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.into_iter().any(|element| other.contains(&element))
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut intersection = Self::with_capacity(10);
        for intersected_element in self.into_iter().filter(|element| other.contains(element)) {
            intersection.add(intersected_element);
        }
        intersection
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut difference = Self::with_capacity(10);
        for nonintersected_element in self.into_iter().filter(|element| !other.contains(element)) {
            difference.add(nonintersected_element);
        }
        difference
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut union = Self::with_capacity(10);
        for element in self.into_iter() {
            union.add(element);
        }
        for element in other.into_iter() {
            union.add(element);
        }
        union
    }

    fn bucket_for(&self, element: T) -> usize {
        Self::hash_for(element) as usize % self.storage.len()
    }

    fn hash_for(element: T) -> u64 {
        let mut hasher = DefaultHasher::new();
        element.hash(&mut hasher);
        hasher.finish()
    }

    fn contained_in_bucket(&self, bucket: usize, element: &T) -> bool {
        self.storage[bucket].contains(element)
    }

    fn add_to_bucket(&mut self, bucket: usize, element: T) {
        self.add_to_buck_no_resize(bucket, element);
        self.resize_if_needed(bucket);
    }

    fn add_to_buck_no_resize(&mut self, bucket: usize, element: T) {
        self.storage[bucket].push(element);
    }

    fn resize_if_needed(&mut self, bucket: usize) {
        let max_bucket_capacity = 10 * (self.storage.len() as f64).log2().powi(2).ceil() as usize;
        if self.storage[bucket].len() > max_bucket_capacity {
            self.resize();
        }
    }

    fn resize(&mut self) {
        let new_size = (self.storage.len() as f64 * 1.5).ceil() as usize;
        self.storage.resize(new_size, Vec::new());
        for idx in 0..self.storage.len() {
            if let false = self.storage[idx].is_empty() {
                let mut old = vec![Vec::new()];
                self.storage[idx..=idx].swap_with_slice(&mut old);
                for &element in old[0].iter() {
                    self.add_to_buck_no_resize(self.bucket_for(element), element);
                }
            }
        }
    }
}

impl<T: Copy + Eq + Hash> IntoIterator for &CustomSet<T> {
    type Item = T;
    type IntoIter = impl Iterator<Item = T>;

    fn into_iter(self) -> Self::IntoIter {
        self.storage
            .iter()
            .flat_map(|v| v.iter().map(|element| *element))
    }
}
