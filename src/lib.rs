mod lz77;

use std::io::Read;
use std::str;

use bitvec::vec::BitVec;
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
    let str = "ababbbabaabab".bytes().collect();

    let res = lz77_compress(&str);

    println!("(siz : {}) compressed payload: {:?}", res.len(), res);

    println!("compression ratio: {}", (str.len() as f64) / (res.len() as f64))
}

#[test]
fn test_uncompress_only() {
    let str: Vec<u8> = vec![132, 133, 152, 138, 113, 97, 57, 98];

    let res = lz77_decompress(&str);

    assert_eq!("ababbbabaabab", str::from_utf8(&res[..]).unwrap());
}