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

#[cfg(feature = "gestalt")]
pub mod gestalt;

#[cfg(feature = "hamming")]
pub mod hamming;

#[cfg(feature = "jaro")]
pub mod jaro;

#[cfg(feature = "levenshtein")]
pub mod levenshtein;

#[cfg(feature = "optimal_string_alignment")]
pub mod optimal_string_alignment;

#[cfg(feature = "sorensen_dice")]
pub mod sorensen_dice;
