//! This library implements string similarity metrics.

#![forbid(unsafe_code)]
#![allow(
    // these casts are sometimes needed. They restrict the length of input iterators
    // but there isn't really any way around this except for always working with
    // 128 bit types
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    // not practical
    clippy::needless_pass_by_value,
    clippy::similar_names,
    // noisy
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    // todo https://github.com/rapidfuzz/strsim-rs/issues/59
    clippy::range_plus_one
)]

use std::char;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::Chars;

pub mod algorithms;
pub use algorithms::damerau_levenshtein::{
    damerau_levenshtein, generic_damerau_levenshtein, normalized_damerau_levenshtein,
};
pub use algorithms::hamming::hamming;
pub use algorithms::jaro::{jaro, jaro_winkler};
pub use algorithms::levenshtein::{generic_levenshtein, levenshtein, normalized_levenshtein};
pub use algorithms::optimal_string_alignment::osa_distance;
pub use algorithms::sorensen_dice::sorensen_dice;

#[derive(Debug, PartialEq)]
pub enum FuzztError {
    DifferentLengthArgs,
}

impl Display for FuzztError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        let text = match self {
            FuzztError::DifferentLengthArgs => "Differing length arguments provided",
        };

        write!(fmt, "{text}")
    }
}

impl Error for FuzztError {}

struct StringWrapper<'a>(&'a str);

impl<'a, 'b> IntoIterator for &'a StringWrapper<'b> {
    type Item = char;
    type IntoIter = Chars<'b>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.chars()
    }
}

/* Returns the final index for a value in a single vector that represents a fixed
2d grid */
fn flat_index(i: usize, j: usize, width: usize) -> usize {
    j * width + i
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct RowId {
    val: isize,
}

impl Default for RowId {
    fn default() -> Self {
        Self { val: -1 }
    }
}

#[derive(Default, Clone)]
struct GrowingHashmapMapElemChar<ValueType> {
    key: u32,
    value: ValueType,
}

/// specialized hashmap to store user provided types
/// this implementation relies on a couple of base assumptions in order to simplify the implementation
/// - the hashmap does not have an upper limit of included items
/// - the default value for the `ValueType` can be used as a dummy value to indicate an empty cell
/// - elements can't be removed
/// - only allocates memory on first write access.
///   This improves performance for hashmaps that are never written to
struct GrowingHashmapChar<ValueType> {
    used: i32,
    fill: i32,
    mask: i32,
    map: Option<Vec<GrowingHashmapMapElemChar<ValueType>>>,
}

impl<ValueType> Default for GrowingHashmapChar<ValueType>
where
    ValueType: Default + Clone + Eq,
{
    fn default() -> Self {
        Self {
            used: 0,
            fill: 0,
            mask: -1,
            map: None,
        }
    }
}

impl<ValueType> GrowingHashmapChar<ValueType>
where
    ValueType: Default + Clone + Eq + Copy,
{
    fn get(&self, key: u32) -> ValueType {
        self.map
            .as_ref()
            .map_or_else(|| Default::default(), |map| map[self.lookup(key)].value)
    }

    fn get_mut(&mut self, key: u32) -> &mut ValueType {
        if self.map.is_none() {
            self.allocate();
        }

        let mut i = self.lookup(key);
        if self
            .map
            .as_ref()
            .expect("map should have been created above")[i]
            .value
            == Default::default()
        {
            self.fill += 1;
            // resize when 2/3 full
            if self.fill * 3 >= (self.mask + 1) * 2 {
                self.grow((self.used + 1) * 2);
                i = self.lookup(key);
            }

            self.used += 1;
        }

        let elem = &mut self
            .map
            .as_mut()
            .expect("map should have been created above")[i];
        elem.key = key;
        &mut elem.value
    }

    fn allocate(&mut self) {
        self.mask = 8 - 1;
        self.map = Some(vec![GrowingHashmapMapElemChar::default(); 8]);
    }

    /// lookup key inside the hashmap using a similar collision resolution
    /// strategy to `CPython` and `Ruby`
    fn lookup(&self, key: u32) -> usize {
        let hash = key;
        let mut i = hash as usize & self.mask as usize;

        let map = self
            .map
            .as_ref()
            .expect("callers have to ensure map is allocated");

        if map[i].value == Default::default() || map[i].key == key {
            return i;
        }

        let mut perturb = key;
        loop {
            i = (i * 5 + perturb as usize + 1) & self.mask as usize;

            if map[i].value == Default::default() || map[i].key == key {
                return i;
            }

            perturb >>= 5;
        }
    }

    fn grow(&mut self, min_used: i32) {
        let mut new_size = self.mask + 1;
        while new_size <= min_used {
            new_size <<= 1;
        }

        self.fill = self.used;
        self.mask = new_size - 1;

        let old_map = std::mem::replace(
            self.map
                .as_mut()
                .expect("callers have to ensure map is allocated"),
            vec![GrowingHashmapMapElemChar::<ValueType>::default(); new_size as usize],
        );

        for elem in old_map {
            if elem.value != Default::default() {
                let j = self.lookup(elem.key);
                let new_elem = &mut self.map.as_mut().expect("map created above")[j];
                new_elem.key = elem.key;
                new_elem.value = elem.value;
                self.used -= 1;
                if self.used == 0 {
                    break;
                }
            }
        }

        self.used = self.fill;
    }
}

struct HybridGrowingHashmapChar<ValueType> {
    map: GrowingHashmapChar<ValueType>,
    extended_ascii: [ValueType; 256],
}

impl<ValueType> HybridGrowingHashmapChar<ValueType>
where
    ValueType: Default + Clone + Copy + Eq,
{
    fn get(&self, key: char) -> ValueType {
        let value = key as u32;
        if value <= 255 {
            let val_u8 = u8::try_from(value).expect("we check the bounds above");
            self.extended_ascii[usize::from(val_u8)]
        } else {
            self.map.get(value)
        }
    }

    fn get_mut(&mut self, key: char) -> &mut ValueType {
        let value = key as u32;
        if value <= 255 {
            let val_u8 = u8::try_from(value).expect("we check the bounds above");
            &mut self.extended_ascii[usize::from(val_u8)]
        } else {
            self.map.get_mut(value)
        }
    }
}

impl<ValueType> Default for HybridGrowingHashmapChar<ValueType>
where
    ValueType: Default + Clone + Copy + Eq,
{
    fn default() -> Self {
        HybridGrowingHashmapChar {
            map: GrowingHashmapChar::default(),
            extended_ascii: [Default::default(); 256],
        }
    }
}

/// Returns an Iterator of char tuples.
fn bigrams(s: &str) -> impl Iterator<Item = (char, char)> + '_ {
    s.chars().zip(s.chars().skip(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bigrams_iterator() {
        let mut bi = bigrams("abcde");

        assert_eq!(Some(('a', 'b')), bi.next());
        assert_eq!(Some(('b', 'c')), bi.next());
        assert_eq!(Some(('c', 'd')), bi.next());
        assert_eq!(Some(('d', 'e')), bi.next());
        assert_eq!(None, bi.next());
    }
}
