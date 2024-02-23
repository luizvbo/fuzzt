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

pub mod damerau_levenshtein;
pub mod gestalt;
pub mod hamming;
pub mod jaro;
pub mod levenshtein;
pub mod optimal_string_alignment;
pub mod sorensen_dice;
