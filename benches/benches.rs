//! Benchmarks for strsim.

#![feature(test)]

extern crate fuzzt;
extern crate test;
use self::test::Bencher;

#[bench]
fn bench_hamming(bencher: &mut Bencher) {
    let a = "ACAAGATGCCATTGTCCCCCGGCCTCCTGCTGCTGCTGCTCTCCGGGG";
    let b = "CCTGGAGGGTGGCCCCACCGGCCGAGACAGCGAGCATATGCAGGAAGC";
    bencher.iter(|| {
        fuzzt::hamming(a, b).unwrap();
    })
}

#[bench]
fn bench_jaro(bencher: &mut Bencher) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";
    bencher.iter(|| {
        fuzzt::jaro(a, b);
    })
}

#[bench]
fn bench_jaro_winkler(bencher: &mut Bencher) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";
    bencher.iter(|| {
        fuzzt::jaro_winkler(a, b);
    })
}

#[bench]
fn bench_levenshtein(bencher: &mut Bencher) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";
    bencher.iter(|| {
        fuzzt::levenshtein(a, b);
    })
}

#[bench]
fn bench_levenshtein_on_u8(bencher: &mut Bencher) {
    bencher.iter(|| {
        fuzzt::generic_levenshtein(&vec![0u8; 30], &vec![7u8; 31]);
    })
}

#[bench]
fn bench_normalized_levenshtein(bencher: &mut Bencher) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";
    bencher.iter(|| {
        fuzzt::normalized_levenshtein(a, b);
    })
}

#[bench]
fn bench_osa_distance(bencher: &mut Bencher) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";
    bencher.iter(|| {
        fuzzt::osa_distance(a, b);
    })
}

#[bench]
fn bench_damerau_levenshtein(bencher: &mut Bencher) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";
    bencher.iter(|| {
        fuzzt::damerau_levenshtein(a, b);
    })
}

#[bench]
fn bench_normalized_damerau_levenshtein(bencher: &mut Bencher) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";
    bencher.iter(|| {
        fuzzt::normalized_damerau_levenshtein(a, b);
    })
}

#[bench]
fn bench_sorensen_dice(bencher: &mut Bencher) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";
    bencher.iter(|| {
        fuzzt::sorensen_dice(a, b);
    })
}
