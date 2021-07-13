#[macro_use]
extern crate std;

use bitarray::BitArray;

fn test_n_words<const B: usize>(words: usize) {
    eprintln!("running with {} bytes and {} words", B, words);
    let mut dict = vec![BitArray::<B>::zeros(); words];
    hamming_dict::generate_dict_rand(&mut dict);

    for word in dict.iter().map(|word| &**word) {
        for &byte in word {
            eprint!("{:02X}", byte);
        }
        eprint!(",");
    }
    eprintln!();
    eprintln!(
        "each word closest distance from all other words: {:?}",
        dict.iter()
            .enumerate()
            .map(|(ix, word)| dict
                .iter()
                .enumerate()
                .filter(|&(other_ix, _)| ix != other_ix)
                .map(|(_, other_word)| other_word.distance(word))
                .min()
                .unwrap())
            .collect::<Vec<_>>()
    );
}

fn main() {
    for p in 1..=8 {
        test_n_words::<64>(1 << p);
    }
}
