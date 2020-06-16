mod challenge_1;
mod challenge_2;
mod challenge_3;
mod challenge_4;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // challenge_1::test_hex_2_base64();
    // challenge_2::test_fixed_xor();
    // challenge_3::single_byte_xor();

    if let Ok(lines) = read_lines("src/challenge_4.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                challenge_4::single_byte_xor(ip);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
