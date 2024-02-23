# Fuzzt

[Rust](https://www.rust-lang.org) implementations of
[string similarity metrics]:

- [Hamming](#hamming)
- [Levenshtein](#levenshtein) (distance & normalized)
- [Optimal string alignment](#optimal-string-alignment)
- [Damerau-Levenshtein](#damerau-levenshtein) (distance & normalized)
- [Jaro and Jaro-Winkler](#jaro-and-jaro-winkler)
- [Sørensen-Dice](#sørensen-dice)
- [Gestalt pattern matching](#gestalt-pattern-matching)

The normalized versions return values between `0.0` and `1.0`, where `1.0` means
an exact match.

There are also generic versions of the functions for non-string inputs.

## What is new?

This crate is heavily based on the
[strsim-rs](https://github.com/rapidfuzz/strsim-rs) crate, with some nice
additions:

- [Gestalt pattern matching](#gestalt-pattern-matching), the algorithm used by
  python difflib SequenceMatcher
- [Top-N matching](#top-n-matching), a method to retrieve the best N matches
  from a collection of choices.
- [Feature selection](#feature-selection), allows you to select only the
  features (metrics) you want to use, reducing the memory footprint of your
  application.

### Top-N Matching

The method `get_top_n` gets a list of the best matches from a collection of
choices. This feature is inspired by the `extractBests` method from the Python
[fuzzywuzzy](https://github.com/seatgeek/fuzzywuzzy) package (now
[thefuzz](https://github.com/seatgeek/thefuzz)).

The `get_top_n` method takes a query string, an array of choice strings, a
cutoff similarity score, an optional number of top matches to return, an
optional string processor, and an optional similarity metric. It processes each
choice and the query using the provided or default string processor, computes
the similarity between the processed query and each processed choice using the
provided or default similarity metric, and returns the top-N matches that have a
similarity score greater than or equal to the cutoff.

Here's the signature of the `get_top_n` method:

```rust
extern crate fuzzt;
use fuzzt::{algorithms::NormalizedLevenshtein, get_top_n, processors::NullStringProcessor};

fn main() {
    let matches = get_top_n(
        "apple",
        &["apply", "apples", "ape", "applet", "applesauce"],
        Some(0.8),
        Some(3),
        Some(&NullStringProcessor),
        Some(&NormalizedLevenshtein),
    );
    assert_eq!(matches, ["apples", "applet", "apply"]);
}
```

### Feature selection

`fuzzt` is designed with flexibility in mind, allowing you to select only the
features you need for your specific use case. This can help to reduce the
footprint of your application and optimize performance.

The crate includes the following features:

- damerau_levenshtein
- gestalt
- hamming
- jaro
- levenshtein
- optimal_string_alignment
- sorensen_dice

By default, all features are included when you add `fuzzt` as a dependency.
However, you can choose to include only specific features by listing them under
the `features` key in your `Cargo.toml` file. For example:

```toml
[dependencies]
fuzzt = { version = "*", default-features = false, features = ["levenshtein", "jaro"] }
```

## Installation

`Fuzzt` is available on [crates.io](https://crates.io/crates/fuzzt). Add it to
your project:

```sh
cargo add fuzzt
```

## Usage

Go to [Docs.rs](https://docs.rs/fuzzt/) for the full documentation. You can also
clone the repo, and run `$ cargo doc --open`.

### Examples

```rust
extern crate fuzzt;

use fuzzt::{
    damerau_levenshtein, hamming, jaro, jaro_winkler, levenshtein, normalized_damerau_levenshtein,
    normalized_levenshtein, osa_distance, sequence_matcher, sorensen_dice,
};

fn main() {
    match hamming("hamming", "hammers") {
        Ok(distance) => assert_eq!(3, distance),
        Err(why) => panic!("{:?}", why),
    }

    assert_eq!(levenshtein("kitten", "sitting"), 3);
    assert!((normalized_levenshtein("kitten", "sitting") - 0.571).abs() < 0.001);
    assert_eq!(osa_distance("ac", "cba"), 3);
    assert_eq!(damerau_levenshtein("ac", "cba"), 2);
    assert!((normalized_damerau_levenshtein("levenshtein", "löwenbräu") - 0.272).abs() < 0.001);
    assert_eq!(jaro("Friedrich Nietzsche", "Jean-Paul Sartre"), 0.3918859649122807);
    assert_eq!(
        jaro_winkler("cheeseburger", "cheese fries"),
        0.8666666666666666
    );
    assert_eq!(
        sorensen_dice("web applications", "applications of the web"),
        0.7878787878787878
    );
    assert_eq!(
        sequence_matcher("this is a test", "this is a test!"),
        0.9655172413793104
    );
}
```

Using the generic versions of the functions:

```rust
extern crate fuzzt;

use fuzzt::generic_levenshtein;

fn main() {
    assert_eq!(2, generic_levenshtein(&[1, 2, 3], &[0, 2, 5]));
}
```

## Algorithms

### Hamming

The Hamming distance between two strings of equal length is the number of
positions at which the corresponding symbols are different. It measures the
minimum number of substitutions required to change one string into the other.

### Levenshtein

The Levenshtein distance is a string metric for measuring the difference between
two sequences. It quantifies how many edits (insertions, deletions, or
substitutions) you need to make to change one string into another. The
normalized version of this metric gives you a proportion between 0 and 1, where
1 means the strings are identical.

### Optimal String Alignment

The Optimal String Alignment (OSA), also known as the restricted
Damerau-Levenshtein distance, computes the shortest distance considering only
adjacent transpositions. This means it doesn't allow substrings to move as a
block, unlike the Damerau-Levenshtein distance.

### Damerau-Levenshtein

Damerau-Levenshtein distance is an extension of the Levenshtein distance,
allowing for transpositions of two adjacent characters along with insertions,
deletions, and substitutions. The normalized version gives a proportion between
0 and 1, where 1 means the strings are identical.

### Jaro and Jaro-Winkler

The Jaro distance allows for transpositions and takes into account the number
and order of common characters between two strings. The Jaro-Winkler distance is
a modification of the Jaro distance that gives more favorable ratings to strings
that match from the beginning.

### Sørensen-Dice

This coefficient is a statistic used to gauge the similarity of two samples.
It's calculated as twice the size of the intersection of the sets, divided by
the sum of the sizes of the two sets.

### Gestalt Pattern Matching

This is the algorithm used by Python's `difflib.SequenceMatcher`. It uses a
heuristic called "Ratcliff/Obershelp" that computes the doubled number of
matching characters divided by the total number of characters in the two
strings. It's particularly good at detecting close matches and some types of
typos.

## Contributing

If you don't want to install Rust itself, you can run `$ ./dev` for a
development CLI if you have [Docker] installed.

Benchmarks require a Nightly toolchain. Run `$ cargo +nightly bench`.

## License

[MIT](https://github.com/luizvbo/fuzzt/blob/main/LICENSE)

[string similarity metrics]: http://en.wikipedia.org/wiki/String_metric
[Damerau-Levenshtein]: http://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
[Jaro and Jaro-Winkler]: http://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance
[Levenshtein]: http://en.wikipedia.org/wiki/Levenshtein_distance
[Hamming]: http://en.wikipedia.org/wiki/Hamming_distance
[Optimal string alignment]: https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance#Optimal_string_alignment_distance
[Sørensen-Dice]: http://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient
[Gestalt pattern matching]: https://en.wikipedia.org/wiki/Gestalt_pattern_matching
[Docker]: https://docs.docker.com/engine/installation/
