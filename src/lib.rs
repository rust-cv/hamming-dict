#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use bitarray::BitArray;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

/// Generate a dictionary with a specific number of `words` as [`BitArray`] with `B` bytes.
///
/// This is deterministic, and thus will always produce the same dictionary given the same
/// `B` and `words` parameters.
///
/// Returns the codewords in the dictionary as a [`Vec`].
#[cfg(feature = "alloc")]
pub fn generate_dict<const B: usize>(words: usize) -> Vec<BitArray<B>> {
    let mut dict = vec![BitArray::zeros(); words];
    generate_dict_rand(&mut dict);
    dict
}

/// Generate a dictionary by mutating a slice of [`BitArray`].
///
/// This is deterministic, and thus will always produce the same dictionary given the same
/// input slice of [`BitArray`] (`dict`).
///
/// It is recommended to pass random bytes as input, or this may take an extraordinarily
/// long time to finish.
///
/// Useful if you want to use your own seed to produce unique codewords.
pub fn generate_dict_from<const B: usize>(dict: &mut [BitArray<B>]) {
    match dict.len() {
        0 => return,
        1 => return,
        2..=64 => {
            generate_dict_stage_2(dict);
            return;
        }
        len => {
            let (left, right) = dict.split_at_mut(len / 2);
            generate_dict_from(left);
            generate_dict_from(right);
        }
    }
    generate_dict_stage_2(dict);
}

/// Generate a deterministic dictionary by mutating a slice of [`BitArray`].
///
/// The actual values of the input slice are ignored, as they are replaced with random values.
/// This is deterministic based on the number of words in the dictionary (length of `dict`) only.
///
/// Internally this uses [`Xoshiro256PlusPlus`], calling [`SeedableRng::seed_from_u64`] with `0` as seed.
pub fn generate_dict_rand<const B: usize>(dict: &mut [BitArray<B>]) {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);
    for word in &mut *dict {
        for byte in &mut **word {
            *byte = rng.gen();
        }
    }
    generate_dict_from(dict);
}

fn generate_dict_stage_2<const B: usize>(dict: &mut [BitArray<B>]) {
    let mut changed = true;
    while changed {
        changed = false;
        for ix in 0..dict.len() {
            'outer: loop {
                // Immutable reference to item.
                let word = &dict[ix];
                // Compute the current sum of all distances from all words in the dictionary from this word.
                let old_closest_distance = closest_distance_sum_distances(dict, word, ix);

                // Go through every one bit mutation of the word.
                for byte in 0..B {
                    for bit in 0..8 {
                        // Mutate the word by one bit.
                        let mut new_word = *word;
                        new_word[byte] ^= 1 << bit;

                        // Check its new distance to all other words.
                        // Subtract `1` because the word now has `1` extra distance from
                        // itself, which shouldn't be counted.
                        let closest_distance = closest_distance_sum_distances(dict, &new_word, ix);

                        // Check if this distance is further from all bitstrings than the previous best.
                        if closest_distance > old_closest_distance {
                            // It was, so set this as the new word.
                            dict[ix] = new_word;
                            changed = true;
                            continue 'outer;
                        }
                    }
                }

                break;
            }
        }
    }
}

fn closest_distance_sum_distances<const B: usize>(
    dict: &[BitArray<B>],
    word: &BitArray<B>,
    ignore: usize,
) -> (u32, u64) {
    let mut closest_distance = u32::MAX;
    let mut sum_distances = 0;
    for distance in dict
        .iter()
        .enumerate()
        .filter(|&(ix, _)| ix != ignore)
        .map(|(_, other_word)| other_word.distance(word))
    {
        sum_distances += distance as u64;
        if distance < closest_distance {
            closest_distance = distance;
        }
    }
    (closest_distance, sum_distances)
}
