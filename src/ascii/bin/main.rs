fn main () {
	ascii_value();
}

const CASE_MASK: u8 = b'a' ^ b'A';
const CASE_MASK2: u8 = b'c' ^ b'C';
fn ascii_value () {
	let a: u8 = b'a';
	let b: u8 = b'A';
	println!(">>>> o valor de CASE_MASK é: {}\n a: {}\n A: {}", CASE_MASK, a, b);

	let a: u8 = b'c';
	let b: u8 = b'C';
	println!(">>>> o valor de CASE_MASK é: {}\n c: {}\n C: {}", CASE_MASK2, a, b);
}
