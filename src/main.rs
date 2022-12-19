mod lz77;

use std::env;
use std::fs;

use minilz77::{lz77_compress, lz77_decompress};

fn main() {
    if env::args().len() < 1 {
        panic!("Error : please enter file");
    }

    match std::fs::read(env::args().nth(1).unwrap()) {
        // File can be read into a vector of bytes
        Ok(bytes) => {
            let original = bytes.len() as f32;
            let str_compressed = lz77_compress(&bytes);
            let compressed = str_compressed.len() as f32;

            println!("Verifying integrity...");
            assert_eq!(lz77_decompress(&str_compressed), bytes);

            println!("Compression ratio : {}", original / compressed);

            println!("Writing compressed contents to file...");
            fs::write(format!("{}.lz77", env::args().nth(1).unwrap()), str_compressed).unwrap();
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
