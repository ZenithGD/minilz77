use minilz77::{lz77_compress, lz77_decompress};

#[test]
fn compress_and_uncompress1() {
    let str: Vec<u8> = "001010210210212021021200".bytes().collect();

    assert_eq!(str, lz77_decompress(&lz77_compress(&str)))
}

#[test]
fn compress_and_uncompress2() {
    let str: Vec<u8> = "001010210210212021021200".bytes().collect();

    assert_eq!(str, lz77_decompress(&lz77_compress(&str)))
}