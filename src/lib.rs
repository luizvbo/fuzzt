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

pub mod algorithms;
pub mod processors;
pub use utils::FuzztError;
mod utils;
mod matcher;

pub use matcher::get_top_n;
