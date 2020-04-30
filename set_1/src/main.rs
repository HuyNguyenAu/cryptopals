fn main() {
    test_hex_2_base64();
    test_fixed_xor();
    single_byte_xor();
}

// Test Challenge 1
fn test_hex_2_base64() {
    let input = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let expected = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    assert_eq!(
        hex_2_base64(input),
        expected,
        "Testing Fixed XOR in Set 1 Challenge 1."
    );
}

// Test Challenge 2
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
fn chi_squared_test(observed: f32, expected: f32) -> f32 {
    (observed - expected).powf(2.0_f32) / expected
}

// https://crypto.stackexchange.com/questions/30209/developing-algorithm-for-detecting-plain-text-via-frequency-analysis
fn english_score(value: &str) -> f32 {
    let letter_frequency = vec![
        0.0804, 0.0148, 0.0334, 0.0382, 0.1249, 0.0240, 0.0187, 0.0505, 0.0757, 0.0016, 0.0054,
        0.0407, 0.0251, 0.0723, 0.0764, 0.0214, 0.0012, 0.0628, 0.0651, 0.0928, 0.0273, 0.0105,
        0.0168, 0.0023, 0.0166, 0.0009,
    ];
    let mut count: Vec<u8> = Vec::new();
    let mut ignored = 0;
    let mut score: f32 = 0_f32;
    for _ in 0..26 {
        count.push(0);
    }
    for c in value.chars() {
        let char_code = c as u8;
        if char_code >= 65 && char_code <= 90 {
            count[(char_code - 65) as usize] += 1;
        } else if char_code >= 97 && char_code <= 122 {
            count[(char_code - 97) as usize] += 1;
        } else if char_code >= 32 && char_code <= 126 || char_code == 9 || char_code == 10 || char_code == 13 {
            ignored += 1;
        } else {
            return -1_f32;
        }
    }

    let len = value.len() - ignored;
    for i in 0..26 {
        let observed = count[i] as f32;
        let expected = (len as f32) * letter_frequency[i];
        
        score += chi_squared_test(observed, expected);
    }
    
    score
}

fn single_byte_xor() {
    let input =
        String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let hex_decoded = hex::decode(input).unwrap();
    let mut results = Vec::<(String, String, f32)>::new();
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
            let result = String::from_utf8(result).unwrap();
            let key = (byte_char as char).to_string();
            let score = english_score(&result);
            results.push((key, result, score));
        }
    }
    
    results.sort_by(|(_, _, s_a), (_, _, s_b)| s_a.partial_cmp(s_b).unwrap());
    
    for (k, v, s) in results {
        println!("{} ({}): {}", k, s, v);
    }
}
