use bitvec::vec::BitVec;
use bitvec::prelude::*;

fn bits_to_phrase(bits : &BitVec<u8>, n : usize) -> (usize, usize, u8) {

    //println!("Got bits {:?}", bits);
    let ln = (n as f32).log2() as usize + 1;
    if bits.len() != 2 * ln + 8 {
        panic!("Length doesn't match");
    } 

    let i : usize = bits[0 .. ln].load::<usize>();
    let l : usize = bits[ln .. 2 * ln].load::<usize>();
    let c : u8    = bits[2 * ln .. 2 * ln + 8].load::<u8>();

    (i, l, c)
}

pub fn read_phrases(bits : &BitVec<u8>) -> Vec<u8> {

    let mut output : Vec<u8> = vec![];

    //println!("bits len : {}", bits.len());

    //bits.into_vec() for bits into a vec<u8>
    let mut n : usize = 1;
    let mut idx : usize = 0;
    loop {
        let ln = (n as f32).log2() as usize + 1;

        if idx + 2 * ln + 8 > bits.len() {
            break;
        }
        
        //println!("n = {}, idx = {}, ln = {}", n, idx, ln);

        let (i, l, s) = bits_to_phrase(&bits.get(idx .. idx + 2 * ln + 8).unwrap().to_bitvec(), n);

        //println!("Found phrase : {:?}", (i, l, s));

        for k in 0 .. l {
            output.push(output[n - i + k - 1]);
        }

        output.push(s);
        idx += 2 * ln + 8;
        n += l + 1;
    }

    return output
}

#[test]
fn test_bits_to_phrase() {
    let bits = bitvec![u8, Lsb0; 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0];

    assert_eq!(bits_to_phrase(&bits, 4), (2, 3, 'A' as u8));

    let bits2 = bitvec![u8, Lsb0; 0, 0, 1, 0, 0, 0, 0, 0, 1, 0];

    assert_eq!(bits_to_phrase(&bits2, 1), (0, 0, 'A' as u8));

    let bits3 = bitvec![u8, Lsb0; 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0];

    assert_eq!(bits_to_phrase(&bits3, 7), (7, 5, 'B' as u8));
}