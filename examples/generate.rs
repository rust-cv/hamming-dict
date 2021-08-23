//! This example generates a file that has code words baked in so they don't need to be generated at runtime.

#[macro_use]
extern crate std;

use bitarray::BitArray;

fn main() {
    let words = 1 << 4;
    let mut dict = vec![BitArray::<64>::zeros(); words];
    hamming_dict::generate_dict_rand(&mut dict);
    println!("use bitarray::BitArray;");
    println!();
    println!("#[rustfmt::skip]");
    println!("pub fn codewords() -> Vec<BitArray<64>> {{");
    println!("  vec![");
    for word in dict {
        print!("        BitArray::new([");
        for byte in word.bytes {
            print!("0x{:02X},", byte);
        }
        println!("]),");
    }
    println!("  ]");
    println!("}}");
}
