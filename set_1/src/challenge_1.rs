// Test Challenge 1
pub fn test_hex_2_base64() {
    let input = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let expected = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    assert_eq!(
        hex_2_base64(input),
        expected,
        "Testing Fixed XOR in Set 1 Challenge 1."
    );
}

// Challenge 1
fn hex_2_base64(value: String) -> String {
    let hex_decoded = hex::decode(value).unwrap();

    base64::encode(hex_decoded)
}