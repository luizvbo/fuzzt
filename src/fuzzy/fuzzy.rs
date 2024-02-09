use crate::{
    fuzzy::interface::{Similarity, SimilarityMetric},
    Levenshtein,
};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use super::processors::{NullStringProcessor, StringProcessor};

/// Returns a list of the best matches to a collection of choices.
///
/// This is a convenience function for getting the choices with the highest scores.
///
/// # Arguments
///
/// * `query` - A string to match against.
/// * `choices` - A list of choices, suitable for use with extract().
/// * `cutoff` - A score threshold. No matches with a score less than this number will be returned.
/// * `n` - Optional maximum for the number of elements returned. Defaults to 3.
/// * `processor` - Optional function for transforming choices before matching. If not provided, `NullStringProcessor` is used.
/// * `scorer` - Optional scoring function for extract(). If not provided, `Levenshtein` is used.
///
/// # Returns
///
/// * A vector of the top 'n' matches from the given choices.
pub fn get_top_n<'a>(
    query: &str,
    choices: &[&'a str],
    cutoff: f32,
    n: Option<usize>,
    processor: Option<&dyn StringProcessor>,
    scorer: Option<&dyn SimilarityMetric>,
) -> Vec<&'a str> {
    let mut matches = BinaryHeap::new();
    let n = n.unwrap_or(3);
    let scorer = match scorer {
        Some(scorer_trait) => scorer_trait,
        None => &Levenshtein,
    };
    let processor = match processor {
        Some(some_processor) => some_processor,
        None => &NullStringProcessor,
    };
    let processed_query = processor.process(&query);

    for &choice in choices {
        let processed_choice = processor.process(&choice);
        let raw_ratio = scorer.compute_metric(processed_query.as_str(), processed_choice.as_str());
        let ratio = match raw_ratio {
            Similarity::Usize(r) => r as f32,
            Similarity::Float(r) => r as f32,
        };
        println!("{:?}", ratio);
        if ratio >= cutoff {
            let int_ratio = match raw_ratio {
                Similarity::Usize(r) => r as i32,
                Similarity::Float(r) => (r * std::u32::MAX as f64) as i32,
            };
            // we're putting the word itself in reverse in so that matches with
            // the same ratio are ordered lexicographically.
            matches.push((-int_ratio, Reverse(choice)));
        }
    }
    println!("{:?}", matches);
    let mut rv = vec![];
    for _ in 0..n {
        if let Some((_, elt)) = matches.pop() {
            rv.push(elt.0);
        } else {
            break;
        }
    }
    rv
}

#[cfg(test)]
mod tests {
    use super::get_top_n;
    use crate::fuzzy::interface::SimilarityMetric;
    use crate::fuzzy::processors::StringProcessor;
    use rstest::rstest;

    #[rstest]
    #[case("hulo", &["hi", "hali", "hoho", "amaz", "auloo", "zulo", "blah", "hopp", "uulo", "aulo", ][..], 0.7, Some(3), None, None, &["aulo", "uulo", "zulo"])]
    fn test_get_top_n<'a>(
        #[case] query: &str,
        #[case] choices: &[&'a str],
        #[case] cutoff: f32,
        #[case] n: Option<usize>,
        #[case] processor: Option<&dyn StringProcessor>,
        #[case] scorer: Option<&dyn SimilarityMetric>,
        #[case] expected: &[&'a str],
    ) {
        let matches = get_top_n(query, choices, cutoff, n, processor, scorer);
        assert_eq!(matches, expected);
    }
}
