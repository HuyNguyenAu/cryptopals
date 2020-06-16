// Test Challenge 2
pub fn test_fixed_xor() {
    let left = String::from("1c0111001f010100061a024b53535009181c");
    let right = String::from("686974207468652062756c6c277320657965");
    let expected = String::from("746865206b696420646f6e277420706c6179");

    assert_eq!(
        fixed_xor(left, right),
        expected,
        "Testing Fixed XOR in Set 1 Challenge 2."
    );
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