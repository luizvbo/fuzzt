use crate::fuzzy::interface::{Similarity, SimilarityMetric};
use crate::utils::FuzztError;
pub type HammingResult = Result<usize, FuzztError>;

/// Calculates the number of positions in the two sequences where the elements
/// differ. Returns an error if the sequences have different lengths.
fn generic_hamming<Iter1, Iter2, Elem1, Elem2>(a: Iter1, b: Iter2) -> HammingResult
where
    Iter1: IntoIterator<Item = Elem1>,
    Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>,
{
    let (mut ita, mut itb) = (a.into_iter(), b.into_iter());
    let mut count = 0;
    loop {
        match (ita.next(), itb.next()) {
            (Some(x), Some(y)) => {
                if x != y {
                    count += 1;
                }
            }
            (None, None) => return Ok(count),
            _ => return Err(FuzztError::DifferentLengthArgs),
        }
    }
}

/// Calculates the number of positions in the two strings where the characters
/// differ. Returns an error if the strings have different lengths.
///
/// ```
/// use fuzzt::{FuzztError::DifferentLengthArgs};
/// use fuzzt::hamming;
///
/// assert_eq!(Ok(3), hamming("hamming", "hammers"));
///
/// assert_eq!(Err(DifferentLengthArgs), hamming("hamming", "ham"));
/// ```
pub fn hamming(a: &str, b: &str) -> HammingResult {
    generic_hamming(a.chars(), b.chars())
}

pub struct Hamming;

impl SimilarityMetric for Hamming {
    fn compute_metric(&self, a: &str, b: &str) -> Similarity {
        Similarity::Usize(hamming(a, b).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_hamming_dist(dist: usize, str1: &str, str2: &str) {
        assert_eq!(Ok(dist), hamming(str1, str2));
    }

    #[test]
    fn hamming_empty() {
        assert_hamming_dist(0, "", "")
    }

    #[test]
    fn hamming_same() {
        assert_hamming_dist(0, "hamming", "hamming")
    }

    #[test]
    fn hamming_numbers() {
        assert_eq!(Ok(1), generic_hamming(&[1, 2, 4], &[1, 2, 3]));
    }

    #[test]
    fn hamming_diff() {
        assert_hamming_dist(3, "hamming", "hammers")
    }

    #[test]
    fn hamming_diff_multibyte() {
        assert_hamming_dist(2, "hamming", "h香mmüng");
    }

    #[test]
    fn hamming_unequal_length() {
        assert_eq!(
            Err(FuzztError::DifferentLengthArgs),
            generic_hamming("ham".chars(), "hamming".chars())
        );
    }

    #[test]
    fn hamming_names() {
        assert_hamming_dist(14, "Friedrich Nietzs", "Jean-Paul Sartre")
    }
}
