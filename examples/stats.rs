#[macro_use]
extern crate std;

use bitarray::BitArray;

fn test_n_words<const B: usize>(words: usize) {
    eprintln!("running with {} bytes and {} words", B, words);
    let mut dict = vec![BitArray::<B>::zeros(); words];
    hamming_dict::generate_dict_from(&mut dict);

    eprintln!(
        "each word average distance from all other words: {:?}",
        dict.iter()
            .map(|word| dict
                .iter()
                .map(|other_word| other_word.distance(word))
                .sum::<u32>() as f64
                / (dict.len() - 1) as f64)
            .collect::<Vec<_>>()
    );
}

fn main() {
    for p in 3..=10 {
        test_n_words::<64>(1 << p);
    }
}
