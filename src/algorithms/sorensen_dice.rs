use crate::algorithms::{Similarity, SimilarityMetric};
use crate::utils::bigrams;
use std::collections::HashMap;

/// Calculates a Sørensen-Dice similarity distance using bigrams.
/// See <https://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient>.
///
/// ```
/// use fuzzt::algorithms::sorensen_dice;
///
/// assert_eq!(1.0, sorensen_dice("", ""));
/// assert_eq!(0.0, sorensen_dice("", "a"));
/// assert_eq!(0.0, sorensen_dice("french", "quebec"));
/// assert_eq!(1.0, sorensen_dice("ferris", "ferris"));
/// assert_eq!(0.8888888888888888, sorensen_dice("feris", "ferris"));
/// ```
pub fn sorensen_dice(a: &str, b: &str) -> f64 {
    // implementation guided by
    // https://github.com/aceakash/string-similarity/blob/f83ba3cd7bae874c20c429774e911ae8cff8bced/src/index.js#L6

    let a: String = a.chars().filter(|&x| !char::is_whitespace(x)).collect();
    let b: String = b.chars().filter(|&x| !char::is_whitespace(x)).collect();

    if a == b {
        return 1.0;
    }

    if a.len() < 2 || b.len() < 2 {
        return 0.0;
    }

    let mut a_bigrams: HashMap<(char, char), usize> = HashMap::new();

    for bigram in bigrams(&a) {
        *a_bigrams.entry(bigram).or_insert(0) += 1;
    }

    let mut intersection_size = 0_usize;

    for bigram in bigrams(&b) {
        a_bigrams.entry(bigram).and_modify(|bi| {
            if *bi > 0 {
                *bi -= 1;
                intersection_size += 1;
            }
        });
    }

    (2 * intersection_size) as f64 / (a.len() + b.len() - 2) as f64
}

pub struct SorensenDice;

impl SimilarityMetric for SorensenDice {
    fn compute_metric(&self, a: &str, b: &str) -> Similarity {
        Similarity::Float(sorensen_dice(a, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sorensen_dice_all() {
        // test cases taken from
        // https://github.com/aceakash/string-similarity/blob/f83ba3cd7bae874c20c429774e911ae8cff8bced/src/spec/index.spec.js#L11

        assert_delta!(1.0, sorensen_dice("a", "a"));
        assert_delta!(0.0, sorensen_dice("a", "b"));
        assert_delta!(1.0, sorensen_dice("", ""));
        assert_delta!(0.0, sorensen_dice("a", ""));
        assert_delta!(0.0, sorensen_dice("", "a"));
        assert_delta!(1.0, sorensen_dice("apple event", "apple    event"));
        assert_delta!(0.90909, sorensen_dice("iphone", "iphone x"));
        assert_delta!(0.0, sorensen_dice("french", "quebec"));
        assert_delta!(1.0, sorensen_dice("france", "france"));
        assert_delta!(0.2, sorensen_dice("fRaNce", "france"));
        assert_delta!(0.8, sorensen_dice("healed", "sealed"));
        assert_delta!(
            0.78788,
            sorensen_dice("web applications", "applications of the web")
        );
        assert_delta!(
            0.92,
            sorensen_dice(
                "this will have a typo somewhere",
                "this will huve a typo somewhere"
            )
        );
        assert_delta!(
            0.60606,
            sorensen_dice(
                "Olive-green table for sale, in extremely good condition.",
                "For sale: table in very good  condition, olive green in colour."
            )
        );
        assert_delta!(
            0.25581,
            sorensen_dice(
                "Olive-green table for sale, in extremely good condition.",
                "For sale: green Subaru Impreza, 210,000 miles"
            )
        );
        assert_delta!(
            0.14118,
            sorensen_dice(
                "Olive-green table for sale, in extremely good condition.",
                "Wanted: mountain bike with at least 21 gears."
            )
        );
        assert_delta!(
            0.77419,
            sorensen_dice("this has one extra word", "this has one word")
        );
    }
}
