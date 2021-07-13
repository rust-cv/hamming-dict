#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use bitarray::BitArray;

/// Generate a dictionary with a specific number of `words` as [`BitArray`] with `B` bytes.
///
/// This is deterministic, and thus will always produce the same dictionary given the same
/// `B` and `words` parameters.
///
/// Returns the words in the dictionary as a [`Vec`].
#[cfg(feature = "alloc")]
pub fn generate_dict<const B: usize>(words: usize) -> Vec<BitArray<B>> {
    let mut dict = vec![BitArray::zeros(); words];
    generate_dict_from(&mut dict);
    dict
}

/// Generate a dictionary by mutating a slice of [`BitArray`].
///
/// This is deterministic, and thus will always produce the same dictionary given the same
/// input slice of [`BitArray`] (`dict`).
pub fn generate_dict_from<const B: usize>(dict: &mut [BitArray<B>]) {
    let mut changed = true;
    while changed {
        changed = false;
        for ix in 0..dict.len() {
            // Immutable reference to item.
            let word = &dict[ix];
            // Compute the current sum of all distances from all words in the dictionary from this word.
            let old_closest_distance = closest_distance(dict, word, ix);

            // Go through every one bit mutation of the word.
            let mut best_closest_distance = old_closest_distance;
            let mut best_word = *word;
            for byte in 0..B {
                for bit in 0..8 {
                    // Mutate the word by one bit.
                    let mut new_word = *word;
                    new_word[byte] ^= 1 << bit;

                    // Check its new distance to all other words.
                    // Subtract `1` because the word now has `1` extra distance from
                    // itself, which shouldn't be counted.
                    let closest_distance = closest_distance(dict, &new_word, ix);

                    // Check if this distance is further from all bitstrings than the previous best.
                    if closest_distance > best_closest_distance {
                        // It was, so set this as the new best.
                        best_closest_distance = closest_distance;
                        best_word = new_word;
                    }
                }
            }

            // Check if the best_net_distance is different.
            if best_closest_distance != old_closest_distance {
                // It must be better then. In this case, we want to note that it changed.
                changed = true;
                // We also need to update the word.
                dict[ix] = best_word;
            }
        }
    }
}

fn closest_distance<const B: usize>(
    dict: &[BitArray<B>],
    word: &BitArray<B>,
    ignore: usize,
) -> u32 {
    dict.iter()
        .enumerate()
        .filter(|&(ix, _)| ix != ignore)
        .map(|(_, other_word)| other_word.distance(word))
        .min()
        .unwrap()
}
