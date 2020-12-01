use code::Code;

mod code;

fn main() {
	let start = 347312;
	let end = 805915;

	let mut code = Code::new(start);
	let mut valid_code_count = 0;
	while code.as_u32() <= end {
		if code.is_valid() {
			valid_code_count += 1;
		}
		code.increment();
	}

	println!("Valid codes in range: {}", valid_code_count);
}