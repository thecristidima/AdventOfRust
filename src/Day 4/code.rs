pub struct Code {
	digits: Vec<u8>
}

impl Code {
	pub fn new(n: u32) -> Code {
		if n < 100_000 || n > 999_999 {
			panic!("Invalid number {}", n);
		}

		let digits = n.to_string()
			.chars()
			.map(|c| c.to_digit(10).unwrap() as u8)
			.collect();

		Code { digits }
	}

	pub fn as_u32(&self) -> u32 {
		self.digits.iter().fold(0, |acc, digit| acc * 10 + *digit as u32)
	}

	pub fn increment(&mut self) {
		let code_as_num = self.as_u32() + 1;
		self.digits = Code::new(code_as_num).digits;
	}

	pub fn is_valid(&self) -> bool {
		self.contains_adjacent_double() && self.has_ascending_digits()
	}

	fn contains_adjacent_double(&self) -> bool { // For part 1 just remove the != comparisons
		self.digits[0] == self.digits[1] && self.digits[1] != self.digits[2]
			|| self.digits[0] != self.digits[1] && self.digits[1] == self.digits[2] && self.digits[2] != self.digits[3]
			|| self.digits[1] != self.digits[2] && self.digits[2] == self.digits[3] && self.digits[3] != self.digits[4]
			|| self.digits[2] != self.digits[3] && self.digits[3] == self.digits[4] && self.digits[4] != self.digits[5]
			|| self.digits[3] != self.digits[4] && self.digits[4] == self.digits[5]
	}

	fn has_ascending_digits(&self) -> bool {
		self.digits[0] <= self.digits[1]
			&& self.digits[1] <= self.digits[2]
			&& self.digits[2] <= self.digits[3]
			&& self.digits[3] <= self.digits[4]
			&& self.digits[4] <= self.digits[5]
	}
}