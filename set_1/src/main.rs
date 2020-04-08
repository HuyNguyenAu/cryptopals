fn main() {
	/*
	let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
	let result = hex_2_base64(hex);

	assert_eq!(result, base64, "We are testing if converting hex to base64 works.");
	*/

	let left = "1c0111001f010100061a024b53535009181c";
	let right = "686974207468652062756c6c277320657965";

	println!("{}", fixed_xor(left, right));		
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
