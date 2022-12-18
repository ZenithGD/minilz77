mod lz77;

use bitvec::view::AsBits;
use lz77::compress::write_phrases;
use lz77::decompress::read_phrases;

///
/// Compress a byte array with the LZ77 algorithm.
///
/// str : The array of bytes to compress.
/// returns : The compressed array
///
pub fn lz77_compress(str: &Vec<u8>) -> Vec<u8> {
    return write_phrases(str).into_vec();
}
///
/// Uncompress a byte array with the LZ77 algorithm.
///
/// str : The array of bytes to uncompress.
/// returns : The uncompressed array
///
pub fn lz77_decompress(str: &Vec<u8>) -> Vec<u8> {
    return read_phrases(&str.as_bits().to_bitvec());
}

#[test]
fn test_compress_only() {
    let str = "aacaacabcabaaac".bytes().collect();

    let res = lz77_compress(&str);

    println!("(siz : {}) compressed payload: {:?}", str.len(), res);

    println!("compression ratio: {}", (str.len() as f64) / (res.len() as f64))
}