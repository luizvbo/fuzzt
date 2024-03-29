#[macro_export]
macro_rules! assert_delta {
    ($x:expr, $y:expr) => {
        assert_delta!($x, $y, 1e-5);
    };
    ($x:expr, $y:expr, $d:expr) => {
        if ($x - $y).abs() > $d {
            panic!(
                "assertion failed: actual: `{}`, expected: `{}`: \
                    actual not within < {} of expected",
                $x, $y, $d
            );
        }
    };
}

#[cfg(feature = "damerau_levenshtein")]
pub mod damerau_levenshtein;
#[cfg(feature = "damerau_levenshtein")]
pub use damerau_levenshtein::{
    damerau_levenshtein, generic_damerau_levenshtein, normalized_damerau_levenshtein,
    DamerauLevenshtein, NormalizedDamerauLevenshtein,
};

pub mod gestalt;
pub use gestalt::{sequence_matcher, SequenceMatcher};

#[cfg(feature = "hamming")]
pub mod hamming;
#[cfg(feature = "hamming")]
pub use hamming::{hamming, Hamming};

#[cfg(feature = "jaro")]
pub mod jaro;
#[cfg(feature = "jaro")]
pub use jaro::{jaro, jaro_winkler, Jaro, JaroWinkler};

#[cfg(feature = "levenshtein")]
pub mod levenshtein;
#[cfg(feature = "levenshtein")]
pub use levenshtein::{
    generic_levenshtein, levenshtein, normalized_levenshtein, Levenshtein, NormalizedLevenshtein,
};

#[cfg(feature = "optimal_string_alignment")]
pub mod optimal_string_alignment;
#[cfg(feature = "optimal_string_alignment")]
pub use optimal_string_alignment::{osa_distance, OSADistance};

#[cfg(feature = "sorensen_dice")]
pub mod sorensen_dice;
#[cfg(feature = "sorensen_dice")]
pub use sorensen_dice::{sorensen_dice, SorensenDice};

pub enum Similarity {
    Usize(usize),
    Float(f64),
}

pub trait SimilarityMetric {
    // The smaller, the more similar 2 strings are.
    fn compute_metric(&self, a: &str, b: &str) -> Similarity;
}
