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
    unimplemented!()
}
