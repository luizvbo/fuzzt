pub enum Similarity {
    Usize(usize),
    Float(f64),
}

pub trait SimilarityMetric {
    // The smaller, the more similar 2 strings are.
    fn compute_metric(&self, a: &str, b: &str) -> Similarity;
}

