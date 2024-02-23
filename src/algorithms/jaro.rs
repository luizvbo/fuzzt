use crate::fuzzy::interface::{Similarity, SimilarityMetric};
use crate::utils::StringWrapper;
use std::cmp::{max, min};

/// Calculates the Jaro similarity between two sequences. The returned value
/// is between 0.0 and 1.0 (higher value means more similar).
fn generic_jaro<'a, 'b, Iter1, Iter2, Elem1, Elem2>(a: &'a Iter1, b: &'b Iter2) -> f64
where
    &'a Iter1: IntoIterator<Item = Elem1>,
    &'b Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>,
{
    let a_len = a.into_iter().count();
    let b_len = b.into_iter().count();

    if a_len == 0 && b_len == 0 {
        return 1.0;
    } else if a_len == 0 || b_len == 0 {
        return 0.0;
    }

    let mut search_range = max(a_len, b_len) / 2;
    search_range = search_range.saturating_sub(1);

    // combine memory allocations to reduce runtime
    let mut flags_memory = vec![false; a_len + b_len];
    let (a_flags, b_flags) = flags_memory.split_at_mut(a_len);

    let mut matches = 0_usize;

    for (i, a_elem) in a.into_iter().enumerate() {
        // prevent integer wrapping
        let min_bound = if i > search_range {
            i - search_range
        } else {
            0
        };

        let max_bound = min(b_len, i + search_range + 1);

        for (j, b_elem) in b.into_iter().enumerate().take(max_bound) {
            if min_bound <= j && a_elem == b_elem && !b_flags[j] {
                a_flags[i] = true;
                b_flags[j] = true;
                matches += 1;
                break;
            }
        }
    }

    let mut transpositions = 0_usize;
    if matches != 0 {
        let mut b_iter = b_flags.iter().zip(b);
        for (a_flag, ch1) in a_flags.iter().zip(a) {
            if *a_flag {
                loop {
                    if let Some((b_flag, ch2)) = b_iter.next() {
                        if !*b_flag {
                            continue;
                        }

                        if ch1 != ch2 {
                            transpositions += 1;
                        }
                        break;
                    }
                }
            }
        }
    }
    transpositions /= 2;

    if matches == 0 {
        0.0
    } else {
        ((matches as f64 / a_len as f64)
            + (matches as f64 / b_len as f64)
            + ((matches - transpositions) as f64 / matches as f64))
            / 3.0
    }
}

/// Like Jaro but gives a boost to sequences that have a common prefix.
fn generic_jaro_winkler<'a, 'b, Iter1, Iter2, Elem1, Elem2>(a: &'a Iter1, b: &'b Iter2) -> f64
where
    &'a Iter1: IntoIterator<Item = Elem1>,
    &'b Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>,
{
    let sim = generic_jaro(a, b);

    if sim > 0.7 {
        let prefix_length = a
            .into_iter()
            .take(4)
            .zip(b)
            .take_while(|(a_elem, b_elem)| a_elem == b_elem)
            .count();

        sim + 0.1 * prefix_length as f64 * (1.0 - sim)
    } else {
        sim
    }
}

/// Calculates the Jaro similarity between two strings. The returned value
/// is between 0.0 and 1.0 (higher value means more similar).
///
/// ```
/// use fuzzt::algorithms::jaro;
///
/// assert!((0.392 - jaro("Friedrich Nietzsche", "Jean-Paul Sartre")).abs() <
///         0.001);
/// ```
pub fn jaro(a: &str, b: &str) -> f64 {
    generic_jaro(&StringWrapper(a), &StringWrapper(b))
}

/// Like Jaro but gives a boost to strings that have a common prefix.
///
/// ```
/// use fuzzt::algorithms::jaro_winkler;
///
/// assert!((0.866 - jaro_winkler("cheeseburger", "cheese fries")).abs() <
///         0.001);
/// ```
pub fn jaro_winkler(a: &str, b: &str) -> f64 {
    generic_jaro_winkler(&StringWrapper(a), &StringWrapper(b))
}

pub struct Jaro;
pub struct JaroWinkler;

impl SimilarityMetric for Jaro {
    fn compute_metric(&self, a: &str, b: &str) -> Similarity {
        Similarity::Float(jaro(a, b))
    }
}

impl SimilarityMetric for JaroWinkler {
    fn compute_metric(&self, a: &str, b: &str) -> Similarity {
        Similarity::Float(jaro_winkler(a, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jaro_both_empty() {
        assert_eq!(1.0, jaro("", ""));
    }

    #[test]
    fn jaro_first_empty() {
        assert_eq!(0.0, jaro("", "jaro"));
    }

    #[test]
    fn jaro_second_empty() {
        assert_eq!(0.0, jaro("distance", ""));
    }

    #[test]
    fn jaro_same() {
        assert_eq!(1.0, jaro("jaro", "jaro"));
    }

    #[test]
    fn jaro_multibyte() {
        assert_delta!(0.818, jaro("testabctest", "testöঙ香test"), 0.001);
        assert_delta!(0.818, jaro("testöঙ香test", "testabctest"), 0.001);
    }

    #[test]
    fn jaro_diff_short() {
        assert_delta!(0.767, jaro("dixon", "dicksonx"), 0.001);
    }

    #[test]
    fn jaro_diff_one_character() {
        assert_eq!(0.0, jaro("a", "b"));
    }

    #[test]
    fn jaro_same_one_character() {
        assert_eq!(1.0, jaro("a", "a"));
    }

    #[test]
    fn generic_jaro_diff() {
        assert_eq!(0.0, generic_jaro(&[1, 2], &[3, 4]));
    }

    #[test]
    fn jaro_diff_one_and_two() {
        assert_delta!(0.83, jaro("a", "ab"), 0.01);
    }

    #[test]
    fn jaro_diff_two_and_one() {
        assert_delta!(0.83, jaro("ab", "a"), 0.01);
    }

    #[test]
    fn jaro_diff_no_transposition() {
        assert_delta!(0.822, jaro("dwayne", "duane"), 0.001);
    }

    #[test]
    fn jaro_diff_with_transposition() {
        assert_delta!(0.944, jaro("martha", "marhta"), 0.001);
        assert_delta!(0.6, jaro("a jke", "jane a k"), 0.001);
    }

    #[test]
    fn jaro_names() {
        assert_delta!(
            0.392,
            jaro("Friedrich Nietzsche", "Jean-Paul Sartre"),
            0.001
        );
    }

    #[test]
    fn jaro_winkler_both_empty() {
        assert_eq!(1.0, jaro_winkler("", ""));
    }

    #[test]
    fn jaro_winkler_first_empty() {
        assert_eq!(0.0, jaro_winkler("", "jaro-winkler"));
    }

    #[test]
    fn jaro_winkler_second_empty() {
        assert_eq!(0.0, jaro_winkler("distance", ""));
    }

    #[test]
    fn jaro_winkler_same() {
        assert_eq!(1.0, jaro_winkler("Jaro-Winkler", "Jaro-Winkler"));
    }

    #[test]
    fn jaro_winkler_multibyte() {
        assert_delta!(0.89, jaro_winkler("testabctest", "testöঙ香test"), 0.001);
        assert_delta!(0.89, jaro_winkler("testöঙ香test", "testabctest"), 0.001);
    }

    #[test]
    fn jaro_winkler_diff_short() {
        assert_delta!(0.813, jaro_winkler("dixon", "dicksonx"), 0.001);
        assert_delta!(0.813, jaro_winkler("dicksonx", "dixon"), 0.001);
    }

    #[test]
    fn jaro_winkler_diff_one_character() {
        assert_eq!(0.0, jaro_winkler("a", "b"));
    }

    #[test]
    fn jaro_winkler_same_one_character() {
        assert_eq!(1.0, jaro_winkler("a", "a"));
    }

    #[test]
    fn jaro_winkler_diff_no_transposition() {
        assert_delta!(0.84, jaro_winkler("dwayne", "duane"), 0.001);
    }

    #[test]
    fn jaro_winkler_diff_with_transposition() {
        assert_delta!(0.961, jaro_winkler("martha", "marhta"), 0.001);
        assert_delta!(0.6, jaro_winkler("a jke", "jane a k"), 0.001);
    }

    #[test]
    fn jaro_winkler_names() {
        assert_delta!(
            0.452,
            jaro_winkler("Friedrich Nietzsche", "Fran-Paul Sartre"),
            0.001
        );
    }

    #[test]
    fn jaro_winkler_long_prefix() {
        assert_delta!(0.866, jaro_winkler("cheeseburger", "cheese fries"), 0.001);
    }

    #[test]
    fn jaro_winkler_more_names() {
        assert_delta!(0.868, jaro_winkler("Thorkel", "Thorgier"), 0.001);
    }

    #[test]
    fn jaro_winkler_length_of_one() {
        assert_delta!(0.738, jaro_winkler("Dinsdale", "D"), 0.001);
    }

    #[test]
    fn jaro_winkler_very_long_prefix() {
        assert_delta!(
            0.98519,
            jaro_winkler("thequickbrownfoxjumpedoverx", "thequickbrownfoxjumpedovery")
        );
    }
}
