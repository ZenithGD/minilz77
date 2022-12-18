mod lz77;

use minilz77::lz77_compress;

fn main() {
    let str = "hi".bytes().collect();
    let compressed = lz77_compress(&str);

    println!("compressed bitvec {:?}", compressed);

    println!("hello")
}
