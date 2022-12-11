pub mod lz77 {
    fn build_phrase(st: &Vec<u8>, n: usize) -> (usize, usize, u8) {
        let (mut idx, mut le): (usize, usize) = (0, 0);

        // loop from the start up to n-1
        for i in 0..n {
            let mut j = 1;
            while j <= n && n + j < st.len() {
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
        (n - idx, le, st[n + le])
    }

    fn all_phrases(str: &Vec<u8>) -> Vec<(usize, usize, u8)> {
        // size of the string in bytes
        let s = str.len();
        let mut v: Vec<(usize, usize, u8)> = vec![];
        // last compressed byte index
        let mut n: usize = 0;
        while n < s {
            let (i, l, c) = build_phrase(str, n);
            v.push((i, l, c));
            n += l + 1;
        }

        return v;
    }
    ///
    /// Compress a byte array with the LZ77 algorithm.
    ///
    /// str : The array of bytes to compress.
    /// returns : The compressed array
    ///
    pub fn lz77_compress(str: &Vec<u8>) -> Vec<u8> {
        let all = all_phrases(str);
        return "str".bytes().collect();
    }
    ///
    /// Uncompress a byte array with the LZ77 algorithm.
    ///
    /// str : The array of bytes to uncompress.
    /// returns : The uncompressed array
    ///
    pub fn lz77_uncompress(str: &Vec<u8>) -> Vec<u8> {
        return "str".bytes().collect();
    }
    // TODO: allow sliding window to start on any array position

    #[test]
    fn test_build_phrase() {
        let str: Vec<u8> = "001010210210212021021200".bytes().collect();

        assert_eq!(build_phrase(&str, 0), (0, 0, '0' as u8));
        assert_eq!(build_phrase(&str, 1), (1, 1, '1' as u8));
        assert_eq!(build_phrase(&str, 3), (2, 3, '2' as u8));
        assert_eq!(build_phrase(&str, 7), (3, 7, '2' as u8));
        assert_eq!(build_phrase(&str, 15), (7, 8, '0' as u8));

        assert_ne!(build_phrase(&str, 14), (7, 8, '0' as u8));
    }

    #[test]
    fn test_all_phrases() {
        let str: Vec<u8> = "001010210210212021021200".bytes().collect();

        assert_eq!(
            all_phrases(&str),
            vec![
                (0, 0, '0' as u8),
                (1, 1, '1' as u8),
                (2, 3, '2' as u8),
                (3, 7, '2' as u8),
                (7, 8, '0' as u8)
            ]
        );
    }
}
