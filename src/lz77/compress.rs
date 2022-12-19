use std::str;

use bitvec::vec::BitVec;
use bitvec::prelude::*;

fn build_phrase(st: &Vec<u8>, n: usize) -> (usize, usize, u8) {
    let (mut idx, mut le): (usize, usize) = (0, 0);

    // loop from the start up to n-2
    for i in 0..n {
        let mut j = 0;
        while j <= n && (n + j) < st.len() {
            // println!("{:?} =? {:?}", &st[i..i + j], &st[n..n + j]);

            // compare substrings
            if &st[i..i + j + 1] != &st[n..n + j + 1] {
                // longest substring from i found, break current loop
                break;
            }

            j += 1;
        }

        // Update longest subsequence starting index and
        if le <= j {
            le = j;
            idx = i;
        }
    }

    if n + le < st.len() {
        return (n - idx, le, st[n + le])
    } else {
        return (n - idx, le - 1, st[n + le - 1] );
    }
}

fn phrase_to_bits((i, l, s): (usize, usize, u8), n: usize) -> BitVec<u8> {
    let ln = (n as f32).log2() as usize + 1;

    let mut bv = bitvec![ u8, Lsb0; 0; 2 * ln + 8 ];

    //println!("creating vector : {:?} (siz : {:?})", bv, bv.len());
    //println!("ln = {}", ln);

    //println!("assigning to [{} : {}] value {}", 0, ln - 1, i);
    bv[0 .. ln].store(i);

    //println!("assigning to [{} : {}] value {}", ln, 2 * ln - 1, l);
    bv[ln .. 2 * ln].store(l);

    //println!("assigning to [{} : {}] value {}", 2 * ln, 2 * ln + 7, s);
    bv[2 * ln .. 2 * ln + 8].store(s);

    //println!("{:?}", bv);
    return bv;
}

pub fn write_phrases(str : &Vec<u8>) -> BitVec<u8> {
    // size of the string in bytes
    let s = str.len();
    let mut v = bitvec![u8, Lsb0;];
    // last compressed byte index
    let mut n: usize = 0;
    while n < s {

        let (i, l, c) = build_phrase(str, n);
        //println!("build phrase {:?} -> {:?}", (i, l, c), str::from_utf8(str.get(n .. n + l + 1).unwrap()));
        let mut bits = phrase_to_bits((i, l, c), n + 1);
        //println!("bits for phrase : {:?}", bits);
        //println!("Appending bits {:?} for phrase {:?}", bits, (i, l, c));
        v.append(&mut bits);
        n += l + 1;
    }

    return v;
}

#[test]
fn test_phrase_to_bits() {
    for i in 1 .. 16 {
        let ln = (i as f32).log2() as usize + 1;
        println!("{} -> {}", i, ln);
    }
    
    let bits = phrase_to_bits((2, 3, 'A' as u8), 4);

    assert_eq!(bits, bitvec![u8, Lsb0; 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0]);

    let bits2 = phrase_to_bits((0, 0, 'A' as u8), 2);

    assert_eq!(bits2, bitvec![u8, Lsb0; 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0]);

    let bits3 = phrase_to_bits((7, 5, 'B' as u8), 7);

    assert_eq!(bits3, bitvec![u8, Lsb0; 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0]);
}
