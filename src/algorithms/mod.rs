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
pub use damerau_levenshtein::{
    damerau_levenshtein, generic_damerau_levenshtein, normalized_damerau_levenshtein,
    DamerauLevenshtein, NormalizedDamerauLevenshtein,
};
#[cfg(feature = "gestalt")]
pub mod gestalt;
pub use gestalt::{sequence_matcher, SequenceMatcher};
#[cfg(feature = "hamming")]
pub mod hamming;
pub use hamming::hamming;
#[cfg(feature = "jaro")]
pub mod jaro;
pub use jaro::{jaro, jaro_winkler};
#[cfg(feature = "levenshtein")]
pub mod levenshtein;
pub use levenshtein::{
    generic_levenshtein, levenshtein, normalized_levenshtein, Levenshtein, NormalizedLevenshtein,
};

#[cfg(feature = "optimal_string_alignment")]
pub mod optimal_string_alignment;
pub use optimal_string_alignment::osa_distance;
#[cfg(feature = "sorensen_dice")]
pub mod sorensen_dice;
pub use sorensen_dice::sorensen_dice;
