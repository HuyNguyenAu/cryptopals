fn main() {
    test_hex_2_base64();
    test_fixed_xor();
    single_byte_xor();
}

// Challenge 1
fn test_hex_2_base64() {
    let input = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let expected = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    assert_eq!(
        hex_2_base64(input),
        expected,
        "Testing Fixed XOR in Set 1 Challenge 1."
    );
}

// Challenge 2
fn test_fixed_xor() {
    let left = String::from("1c0111001f010100061a024b53535009181c");
    let right = String::from("686974207468652062756c6c277320657965");
    let expected = String::from("746865206b696420646f6e277420706c6179");

    assert_eq!(
        fixed_xor(left, right),
        expected,
        "Testing Fixed XOR in Set 1 Challenge 2."
    );
}

// Challenge 1
fn hex_2_base64(value: String) -> String {
    let hex_decoded = hex::decode(value).unwrap();

    base64::encode(hex_decoded)
}

// Challenge 2
fn fixed_xor(left: String, right: String) -> String {
    if left.len() != right.len() {
        return String::from("");
    }

    let left = hex::decode(left).unwrap();
    let right = hex::decode(right).unwrap();

    // For each left side byte, XOR with the corresponsing right side byte.
    // ie. left[n] XOR right[n], n = 0, ..., length(left).   
    let result: Vec<u8> = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| l ^ r)
        .collect();

    hex::encode(result)
}

// Challenge 3
fn single_byte_xor() {
    let input =
        String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let hex_decoded = hex::decode(input).unwrap();
    
    // Loop from ASCII byte 0x20(SPACE) to 0x7E(~).
    for byte_char in 20_u8..127_u8 {
        let mut result: Vec<u8> = Vec::new();
        
        // For each byte in the input, XOR it with the current ASCII byte.
        for byte_hex in &hex_decoded {
            result.push(byte_hex ^ byte_char);
        }
        
        // We will only print the results that have human readable ASCII bytes.
        let mut valid = true;
        
        // Check if the string is a valid plaintext. First we will consider a valid
        // plaintext, when it contains only 0x20 to 0x7E bytes.
        for byte_result in &result {
            // Not sure why byte_result < &32_u8 || byte_result > &126_u8 yeilds true, when byte_result = 32_u8.
            if !(byte_result > &31_u8 && byte_result < &127_u8) {
                valid = false;
                break;
            }
        }
        
        if valid {
            println!("{}: {}", (byte_char) as char, String::from_utf8(result).unwrap());
        }
    }
}
