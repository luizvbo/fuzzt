use crate::fuzzy::interface::{Similarity, SimilarityMetric};

use std::collections::HashMap;

/// Compares two strings `s1` and `s2` and returns a measure of their similarity as a float in the range [0, 1].
///
/// The returned measure is computed as follows:
/// 1. If the total length of the two strings is 0, the function returns 1.0.
/// 2. Otherwise, it computes the intersection of the character counts of the two strings,
///    sums up the counts in the intersection, and returns the ratio of twice the sum of the counts to the total length.
///
/// # Arguments
///
/// * `s1` - The first string to compare.
/// * `s2` - The second string to compare.
///
/// # Returns
///
/// * A float between 0 and 1 representing the similarity of the two strings.
pub fn sequence_matcher(s1: &str, s2: &str) -> f64 {
    let length = s1.len() + s2.len();

    if length == 0 {
        return 1.0;
    }

    let intersect = intersect(&counter(s1), &counter(s2));
    let matches: usize = intersect.values().sum();
    2.0 * (matches as f64) / (length as f64)
}
fn counter(s: &str) -> HashMap<char, usize> {
    let mut count = HashMap::new();
    for c in s.chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    count
}

fn intersect(map1: &HashMap<char, usize>, map2: &HashMap<char, usize>) -> HashMap<char, usize> {
    let mut intersect = HashMap::new();
    for (k, v) in map1 {
        if let Some(v2) = map2.get(k) {
            intersect.insert(*k, *v.min(v2));
        }
    }
    intersect
}

pub struct SequenceMatcher;

impl SimilarityMetric for SequenceMatcher {
    fn compute_metric(&self, a: &str, b: &str) -> Similarity {
        Similarity::Float(sequence_matcher(a, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_ratio() {
        assert_eq!(sequence_matcher("test", "test"), 1.0);
        assert_eq!(sequence_matcher("test", "tent"), 0.75);
        assert_eq!(sequence_matcher("kitten", "sitting"), 0.6153846153846154);
        assert_eq!(sequence_matcher("", ""), 1.0);
        assert_eq!(sequence_matcher("test", ""), 0.0);
        assert_eq!(sequence_matcher("", "test"), 0.0);
    }
}
