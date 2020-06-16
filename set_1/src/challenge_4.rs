// Challenge 4
// Return chi squared test score.
fn chi_squared_test(observed: f32, expected: f32) -> f32 {
    (observed - expected).powf(2.0_f32) / expected
}

// Return a score for how english like a given string is. Code was taken and adapted from
// https://crypto.stackexchange.com/questions/30209/developing-algorithm-for-detecting-plain-text-via-frequency-analysis.
fn english_score(value: &str) -> f32 {
    // List list of english letter frequencies as a percentage. This is taken from
    // https://norvig.com/mayzner.html.
    let letter_frequency = vec![
        0.0804, 0.0148, 0.0334, 0.0382, 0.1249, 0.0240, 0.0187, 0.0505, 0.0757, 0.0016, 0.0054,
        0.0407, 0.0251, 0.0723, 0.0764, 0.0214, 0.0012, 0.0628, 0.0651, 0.0928, 0.0273, 0.0105,
        0.0168, 0.0023, 0.0166, 0.0009,
    ];
    // A list to store the frequency of a - z. This is case insensitive.
    let mut count: Vec<u8> = Vec::new();
    // Keep track of the number of ignored bytes.
    let mut ignored = 0;
    //
    let mut score: f32 = 0_f32;
    for _ in 0..26 {
        count.push(0);
    }
    for c in value.bytes() {
        // Count the number of A-Z chars.
        if c >= 65 && c <= 90 {
            count[(c - 65) as usize] += 1;
        // Count the number of a-z chars.
        } else if c >= 97 && c <= 122 {
            count[(c - 97) as usize] += 1;
        // Ignore everthing else that is not aA-zZ.
        } else if c >= 32 && c <= 126 || c == 9 || c == 10 || c == 13 {
            ignored += 1;
        } else {
            return -1_f32;
        }
    }

    // Calculate the numbber of valid chars.
    let length = value.len() - ignored;

    // For each letter, calculate the chi squared score.
    for i in 0..26 {
        let observed = count[i] as f32;
        let expected = (length as f32) * letter_frequency[i];

        score += chi_squared_test(observed, expected);
    }

    score
}

pub fn single_byte_xor(input: String) {
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
            // Convert the list of bytes into a string.
            let result = String::from_utf8(result).unwrap();
            // Convert key into a string.
            let key = (byte_char as char).to_string();
            // Get the score of the result.
            let score = english_score(&result);
            results.push((key, result, score));
        }
    }

    // Sort the array from lowest score to highest.
    results.sort_by(|(_, _, s_a), (_, _, s_b)| s_a.partial_cmp(s_b).unwrap());

    // Print all results from lowest to highest score.
    for (k, v, s) in results {
        println!("{} ({}): {}", k, s, v);
    }
}
