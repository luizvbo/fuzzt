use crate::fuzzy::interface::{Similarity, SimilarityMetric};
use crate::utils::{flat_index, HybridGrowingHashmapChar, RowId};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::hash::Hash;
use std::mem;

/// Like optimal string alignment, but substrings can be edited an unlimited
/// number of times, and the triangle inequality holds.
///
/// ```
/// use fuzzt::generic_damerau_levenshtein;
///
/// assert_eq!(2, generic_damerau_levenshtein(&[1,2], &[2,3,1]));
/// ```
pub fn generic_damerau_levenshtein<Elem>(a_elems: &[Elem], b_elems: &[Elem]) -> usize
where
    Elem: Eq + Hash + Clone,
{
    let a_len = a_elems.len();
    let b_len = b_elems.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let width = a_len + 2;
    let mut distances = vec![0; (a_len + 2) * (b_len + 2)];
    let max_distance = a_len + b_len;
    distances[0] = max_distance;

    for i in 0..(a_len + 1) {
        distances[flat_index(i + 1, 0, width)] = max_distance;
        distances[flat_index(i + 1, 1, width)] = i;
    }

    for j in 0..(b_len + 1) {
        distances[flat_index(0, j + 1, width)] = max_distance;
        distances[flat_index(1, j + 1, width)] = j;
    }

    let mut elems: HashMap<Elem, usize> = HashMap::with_capacity(64);

    for i in 1..(a_len + 1) {
        let mut db = 0;

        for j in 1..(b_len + 1) {
            let k = match elems.get(&b_elems[j - 1]) {
                Some(&value) => value,
                None => 0,
            };

            let insertion_cost = distances[flat_index(i, j + 1, width)] + 1;
            let deletion_cost = distances[flat_index(i + 1, j, width)] + 1;
            let transposition_cost =
                distances[flat_index(k, db, width)] + (i - k - 1) + 1 + (j - db - 1);

            let mut substitution_cost = distances[flat_index(i, j, width)] + 1;
            if a_elems[i - 1] == b_elems[j - 1] {
                db = j;
                substitution_cost -= 1;
            }

            distances[flat_index(i + 1, j + 1, width)] = min(
                substitution_cost,
                min(insertion_cost, min(deletion_cost, transposition_cost)),
            );
        }

        elems.insert(a_elems[i - 1].clone(), i);
    }

    distances[flat_index(a_len + 1, b_len + 1, width)]
}

fn damerau_levenshtein_impl<Iter1, Iter2>(s1: Iter1, len1: usize, s2: Iter2, len2: usize) -> usize
where
    Iter1: Iterator<Item = char> + Clone,
    Iter2: Iterator<Item = char> + Clone,
{
    // The implementations is based on the paper
    // `Linear space string correction algorithm using the Damerau-Levenshtein distance`
    // from Chunchun Zhao and Sartaj Sahni
    //
    // It has a runtime complexity of `O(N*M)` and a memory usage of `O(N+M)`.
    let max_val = max(len1, len2) as isize + 1;

    let mut last_row_id = HybridGrowingHashmapChar::<RowId>::default();

    let size = len2 + 2;
    let mut fr = vec![max_val; size];
    let mut r1 = vec![max_val; size];
    let mut r: Vec<isize> = (max_val..max_val + 1)
        .chain(0..(size - 1) as isize)
        .collect();

    for (i, ch1) in s1.enumerate().map(|(i, ch1)| (i + 1, ch1)) {
        mem::swap(&mut r, &mut r1);
        let mut last_col_id: isize = -1;
        let mut last_i2l1 = r[1];
        r[1] = i as isize;
        let mut t = max_val;

        for (j, ch2) in s2.clone().enumerate().map(|(j, ch2)| (j + 1, ch2)) {
            let diag = r1[j] + isize::from(ch1 != ch2);
            let left = r[j] + 1;
            let up = r1[j + 1] + 1;
            let mut temp = min(diag, min(left, up));

            if ch1 == ch2 {
                last_col_id = j as isize; // last occurence of s1_i
                fr[j + 1] = r1[j - 1]; // save H_k-1,j-2
                t = last_i2l1; // save H_i-2,l-1
            } else {
                let k = last_row_id.get(ch2).val;
                let l = last_col_id;

                if j as isize - l == 1 {
                    let transpose = fr[j + 1] + (i as isize - k);
                    temp = min(temp, transpose);
                } else if i as isize - k == 1 {
                    let transpose = t + (j as isize - l);
                    temp = min(temp, transpose);
                }
            }

            last_i2l1 = r[j + 1];
            r[j + 1] = temp;
        }
        last_row_id.get_mut(ch1).val = i as isize;
    }

    r[len2 + 1] as usize
}

/// Like optimal string alignment, but substrings can be edited an unlimited
/// number of times, and the triangle inequality holds.
///
/// ```
/// use fuzzt::damerau_levenshtein;
///
/// assert_eq!(2, damerau_levenshtein("ab", "bca"));
/// ```
pub fn damerau_levenshtein(a: &str, b: &str) -> usize {
    damerau_levenshtein_impl(a.chars(), a.chars().count(), b.chars(), b.chars().count())
}

/// Calculates a normalized score of the Damerau–Levenshtein algorithm between
/// 0.0 and 1.0 (inclusive), where 1.0 means the strings are the same.
///
/// ```
/// use fuzzt::normalized_damerau_levenshtein;
///
/// assert!((normalized_damerau_levenshtein("levenshtein", "löwenbräu") - 0.27272).abs() < 0.00001);
/// assert!((normalized_damerau_levenshtein("", "") - 1.0).abs() < 0.00001);
/// assert!(normalized_damerau_levenshtein("", "flower").abs() < 0.00001);
/// assert!(normalized_damerau_levenshtein("tree", "").abs() < 0.00001);
/// assert!((normalized_damerau_levenshtein("sunglasses", "sunglasses") - 1.0).abs() < 0.00001);
/// ```
pub fn normalized_damerau_levenshtein(a: &str, b: &str) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }

    let len1 = a.chars().count();
    let len2 = b.chars().count();
    let dist = damerau_levenshtein_impl(a.chars(), len1, b.chars(), len2);
    1.0 - (dist as f64) / (max(len1, len2) as f64)
}

pub struct DamerauLevenshtein;
pub struct NormalizedDamerauLevenshtein;

impl SimilarityMetric for DamerauLevenshtein {
    fn compute_metric(&self, a: &str, b: &str) -> Similarity {
        Similarity::Usize(damerau_levenshtein(a, b))
    }
}

impl SimilarityMetric for NormalizedDamerauLevenshtein {
    fn compute_metric(&self, a: &str, b: &str) -> Similarity {
        Similarity::Float(normalized_damerau_levenshtein(a, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damerau_levenshtein_empty() {
        assert_eq!(0, damerau_levenshtein("", ""));
    }

    #[test]
    fn damerau_levenshtein_same() {
        assert_eq!(0, damerau_levenshtein("damerau", "damerau"));
    }

    #[test]
    fn damerau_levenshtein_first_empty() {
        assert_eq!(7, damerau_levenshtein("", "damerau"));
    }

    #[test]
    fn damerau_levenshtein_second_empty() {
        assert_eq!(7, damerau_levenshtein("damerau", ""));
    }

    #[test]
    fn damerau_levenshtein_diff() {
        assert_eq!(2, damerau_levenshtein("ca", "abc"));
    }

    #[test]
    fn damerau_levenshtein_diff_short() {
        assert_eq!(3, damerau_levenshtein("damerau", "aderua"));
    }

    #[test]
    fn damerau_levenshtein_diff_reversed() {
        assert_eq!(3, damerau_levenshtein("aderua", "damerau"));
    }

    #[test]
    fn damerau_levenshtein_diff_multibyte() {
        assert_eq!(3, damerau_levenshtein("öঙ香", "abc"));
        assert_eq!(3, damerau_levenshtein("abc", "öঙ香"));
    }

    #[test]
    fn damerau_levenshtein_diff_unequal_length() {
        assert_eq!(6, damerau_levenshtein("damerau", "aderuaxyz"));
    }

    #[test]
    fn damerau_levenshtein_diff_unequal_length_reversed() {
        assert_eq!(6, damerau_levenshtein("aderuaxyz", "damerau"));
    }

    #[test]
    fn damerau_levenshtein_diff_comedians() {
        assert_eq!(5, damerau_levenshtein("Stewart", "Colbert"));
    }

    #[test]
    fn damerau_levenshtein_many_transpositions() {
        assert_eq!(4, damerau_levenshtein("abcdefghijkl", "bacedfgihjlk"));
    }

    #[test]
    fn damerau_levenshtein_diff_longer() {
        let a = "The quick brown fox jumped over the angry dog.";
        let b = "Lehem ipsum dolor sit amet, dicta latine an eam.";
        assert_eq!(36, damerau_levenshtein(a, b));
    }

    #[test]
    fn damerau_levenshtein_beginning_transposition() {
        assert_eq!(1, damerau_levenshtein("foobar", "ofobar"));
    }

    #[test]
    fn damerau_levenshtein_end_transposition() {
        assert_eq!(1, damerau_levenshtein("specter", "spectre"));
    }

    #[test]
    fn damerau_levenshtein_unrestricted_edit() {
        assert_eq!(3, damerau_levenshtein("a cat", "an abct"));
    }

    #[test]
    fn normalized_damerau_levenshtein_diff_short() {
        assert_delta!(
            0.27272,
            normalized_damerau_levenshtein("levenshtein", "löwenbräu")
        );
    }

    #[test]
    fn normalized_damerau_levenshtein_for_empty_strings() {
        assert_delta!(1.0, normalized_damerau_levenshtein("", ""));
    }

    #[test]
    fn normalized_damerau_levenshtein_first_empty() {
        assert_delta!(0.0, normalized_damerau_levenshtein("", "flower"));
    }

    #[test]
    fn normalized_damerau_levenshtein_second_empty() {
        assert_delta!(0.0, normalized_damerau_levenshtein("tree", ""));
    }

    #[test]
    fn normalized_damerau_levenshtein_identical_strings() {
        assert_delta!(
            1.0,
            normalized_damerau_levenshtein("sunglasses", "sunglasses")
        );
    }
}
