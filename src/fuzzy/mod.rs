use crate::{
    fuzzy::interface::{Similarity, SimilarityMetric},
    NormalizedLevenshtein,
};
use processors::{NullStringProcessor, StringProcessor};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub mod interface;
pub mod processors;

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
    cutoff: f64,
    n: Option<usize>,
    processor: Option<&dyn StringProcessor>,
    scorer: Option<&dyn SimilarityMetric>,
) -> Vec<&'a str> {
    let mut matches = BinaryHeap::new();
    let n = n.unwrap_or(3);
    let scorer = match scorer {
        Some(scorer_trait) => scorer_trait,
        None => &NormalizedLevenshtein,
    };
    let processor = match processor {
        Some(some_processor) => some_processor,
        None => &NullStringProcessor,
    };
    let processed_query = processor.process(query);

    for &choice in choices {
        let processed_choice = processor.process(choice);
        let raw_ratio = scorer.compute_metric(processed_query.as_str(), processed_choice.as_str());
        let ratio = match raw_ratio {
            Similarity::Usize(r) => r as f64,
            Similarity::Float(r) => r,
        };
        if ratio >= cutoff {
            let int_ratio = match raw_ratio {
                Similarity::Usize(r) => r as i64,
                Similarity::Float(r) => (r * std::u32::MAX as f64) as i64,
            };
            // we're putting the word itself in reverse in so that matches with
            // the same ratio are ordered lexicographically.
            matches.push((int_ratio, Reverse(choice)));
        }
    }
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
    use crate::algorithms::jaro::JaroWinkler;
    use crate::fuzzy::interface::SimilarityMetric;
    use crate::fuzzy::processors::{LowerAlphaNumStringProcessor, StringProcessor};
    use rstest::rstest;

    #[rstest]
    #[case(0.7, Some(3), None, None, &["brazil", "braziu", "trazil"])]
    #[case(0.9, Some(5), None, None, &["brazil"])]
    #[case(0.7, Some(2), None, Some(&JaroWinkler as &dyn SimilarityMetric), &["brazil", "braziu"])]
    #[case(0.7, Some(2), Some(&LowerAlphaNumStringProcessor as &dyn StringProcessor), None, &["brazil", "BRA ZIL"])]
    fn test_get_top_n<'a>(
        #[case] cutoff: f64,
        #[case] n: Option<usize>,
        #[case] processor: Option<&dyn StringProcessor>,
        #[case] scorer: Option<&dyn SimilarityMetric>,
        #[case] expected: &[&'a str],
    ) {
        let choices = &["trazil", "BRA ZIL", "brazil", "spain", "braziu"][..];
        let query = "brazil";
        let matches = get_top_n(query, choices, cutoff, n, processor, scorer);
        assert_eq!(matches, expected);
    }
}
