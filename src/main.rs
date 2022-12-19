mod lz77;

use std::env;

use minilz77::{lz77_compress, lz77_decompress};

fn main() {
    if env::args().len() < 1 {
        panic!("Error : please enter file");
    }

    match std::fs::read(env::args().nth(1).unwrap()) {
        Ok(bytes) => {
            let original = bytes.len() as f32;
            let str_compressed = lz77_compress(&bytes);
            let compressed = str_compressed.len() as f32;

            println!("Compression ratio : {}", original / compressed);

            assert_eq!(lz77_decompress(&str_compressed), bytes);
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("please run again with appropriate permissions.");
                return;
            }
            panic!("{}", e);
        }
    }
}
