use bitvec::ptr::read;
use bitvec::vec::BitVec;
use bitvec::prelude::*;

fn bits_to_phrase(bits : &BitVec, n : u32) -> (usize, usize, u8) {

    let ln = (n as f32).log2().ceil() as usize;

    println!("ln = {}", ln);
    if bits.len() != 2 * ln + 8 {
        panic!("Length doesn't match");
    } 

    let i : usize = bits[0 .. ln].load::<usize>();
    let l : usize = bits[ln .. 2 * ln].load::<usize>();
    let c : u8    = bits[2 * ln .. 2 * ln + 8].load::<u8>();

    (i, l, c)
}

pub fn read_phrases(bits : &BitVec<u8>) -> Vec<u8> {

    let output : Vec<u8> = vec![];

    //bits.into_vec() for bits into a vec<u8>

    return output
}

#[test]
fn test_bits_to_phrase() {
    let bits = bitvec![0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0];

    assert_eq!(bits_to_phrase(&bits, 4), (2, 3, 'A' as u8));

    let bits2 = bitvec![0, 0, 1, 0, 0, 0, 0, 0, 1, 0];

    assert_eq!(bits_to_phrase(&bits2, 1), (0, 0, 'A' as u8));

    let bits3 = bitvec![1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0];

    assert_eq!(bits_to_phrase(&bits3, 7), (7, 5, 'B' as u8));
}