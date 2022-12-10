use minilz77::lz77::{lz77_compress, lz77_uncompress};

#[test]
fn compress_and_uncompress1() {
    let s1 : &[u8] = "aaa111".as_bytes();

    assert_eq!(s1, lz77_uncompress(lz77_compress(s1)))
}

#[test]
fn compress_and_uncompress2() {
    let s2 : &[u8] = "aaaaaa111aaaabbbb".as_bytes();

    assert_eq!(s2, lz77_uncompress(lz77_compress(s2)))
}
