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

mod algorithms;
mod utils;

pub mod fuzzy;
pub use algorithms::damerau_levenshtein::{
    damerau_levenshtein, generic_damerau_levenshtein, normalized_damerau_levenshtein,
    DamerauLevenshtein, NormalizedDamerauLevenshtein,
};
pub use algorithms::gestalt::sequence_matcher;
pub use algorithms::gestalt::SequenceMatcher;
pub use algorithms::hamming::hamming;
pub use algorithms::jaro::{jaro, jaro_winkler};
pub use algorithms::levenshtein::{
    generic_levenshtein, levenshtein, normalized_levenshtein, Levenshtein, NormalizedLevenshtein,
};
pub use algorithms::optimal_string_alignment::osa_distance;
pub use algorithms::sorensen_dice::sorensen_dice;
pub use utils::FuzztError;
