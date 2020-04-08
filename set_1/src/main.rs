fn main() {
	test_hex_2_base64();
	test_fixed_xor();
}

fn test_hex_2_base64() {
	let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

	assert_eq!(hex_2_base64(input), expected, "Testing Fixed XOR in Set 1 Challenge 1.");
}

fn test_fixed_xor() {
	let left = "1c0111001f010100061a024b53535009181c";
	let right = "686974207468652062756c6c277320657965";
	let expected = "746865206b696420646f6e277420706c6179";
	let result = fixed_xor(left, right);

	assert_eq!(fixed_xor(left, right), expected, "Testing Fixed XOR in Set 1 Challenge 2.");
}

fn hex_2_base64(value: &str) -> String {
	let hex_decoded = hex::decode(value); 
	let hex_decoded = match hex_decoded {
		Ok(result) => result,
		Err(error) => {
			panic!("Unable to decode hex! {}", error);
		}
	}; 

	return base64::encode(hex_decoded);
}

fn fixed_xor(left: &str, right: &str) -> String {
	let mut result: Vec<u8> = Vec::new();

	if left.len() == right.len() {
		let left = hex::decode(left).unwrap();
		let right = hex::decode(right).unwrap();
		
		result  = left.into_iter().zip(right.into_iter()).map(|(l, r)| l ^ r).collect();
	}

	return hex::encode(result);
}
