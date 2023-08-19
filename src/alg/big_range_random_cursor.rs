use std::{collections::HashMap, ops::RangeInclusive};
use crate::randomness;
// - TODO: integrate w. range inclusive.
// - TODO: should be serializable.

/// Goes through a big range (max from 0 to 2^64 - 1), at random
/// without repetition.
pub(crate) struct BigRangeRandomCursor {
    lower_bound: u64,
    upper_bound: u64,
    generated_count: u64,
    hashmap: HashMap<u64, u64>
}

impl BigRangeRandomCursor {
    pub(crate) fn new(range: RangeInclusive<u64>, preexisting: &[u64]) -> Self {
        let mut this = Self { lower_bound: *range.start(), upper_bound: *range.end(), generated_count: 0, hashmap: HashMap::new() };

        assert!(preexisting.len() < 100);
        preexisting.into_iter().for_each(|val| { this.next_(*val); });

        this
    }

    pub(crate) fn iter_mut(&mut self) -> IterMut {
        return IterMut { cursor: self }
    }

    fn next(&mut self) -> Option<u64> {

        // - TODO: use frand here
        self.next_(7)
    }

    fn next_(&mut self, rng_number: u64) -> Option<u64> {
        if self.lower_bound > self.upper_bound {
            return None;
        }

        let r = rng_number;

        let result: u64;

        if let Some(saved) = self.hashmap.get(&r) {
            // this is actually some min, which hasn't been yet returned.
            result = *saved;
        } else {
            result = r;
        }

        // current min is bigger than previous
        // saved value there is previous range min.

        // r is going to become some range min.
        // each range min will be returned once

        // [&self.lower_bound]
        if let Some(previous_range_min) = self.hashmap.get(&self.lower_bound) {
            // current lower_bound has already been encountered
            // and returned (it was "r" selected at some point,
            // at that time either hashmap[r] == None, and r was returned, or it wasn't None - in that case it was
            // returned a step before - at some point hashmap[r] had to be None.)
            self.hashmap.insert(r, *previous_range_min);
        } else {
            self.hashmap.insert(r, self.lower_bound);
        }

        if self.hashmap.len() > 5000 {
            // all keys < lower_bound will not be addressed.

            // - TODO: omg.
            //self.hashmap = self.hashmap.drain_filter(|k, _| { *k >= self.lower_bound }).collect();
        }

        self.generated_count += 1;
        self.lower_bound += 1;
        return Some(result)
    }
}

pub(crate) struct IterMut<'a> {
    cursor: &'a mut BigRangeRandomCursor
}

// implicit elided lifetime. are we good here?
impl Iterator for IterMut<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.next()
    }
}