#[derive(PartialEq)]
enum OpcodeResult {
	Ok,
	Halt,
}

pub struct Calculator {
	ip: usize,
	memory: Vec<i32>,

	input: i32,
}

impl Calculator {
	pub fn new() -> Calculator {
		Calculator {
			ip: 0,
			memory: Vec::<i32>::new(),
			input: 0,
		}
	}

	pub fn load_memory(&mut self, memory: &mut [i32]) {
		self.memory = Vec::from(memory);
	}

	pub fn set_input(&mut self, input: i32) {
		self.input = input;
	}

	pub fn run(&mut self) {
		self.ip = 0;
		loop {
			let res = self.run_opcode();
			if res != OpcodeResult::Ok { break; }
		}
	}

	pub fn dump_memory(&self) -> Vec<i32> {
		self.memory.clone()
	}

	fn run_opcode(&mut self) -> OpcodeResult {
		match self.memory[self.ip as usize] % 100 {
			1 => self.add(),
			2 => self.multiply(),
			3 => self.load_input(),
			4 => self.print_output(),
			5 => self.jump_if_true(),
			6 => self.jump_if_false(),
			7 => self.less_than(),
			8 => self.equals(),
			99 => OpcodeResult::Halt,
			_ => panic!("Unknown opcode {}", self.memory[self.ip as usize])
		}
	}

	fn get_pmodes(&self) -> (i32, i32, i32) {
		let opcode = self.memory[self.ip];
		(opcode / 100 % 10, opcode / 1000 % 10, opcode / 10000 % 10)
	}

	fn get_noun(&self) -> i32 {
		let mem = &self.memory;
		let (pmode_noun, _, _) = self.get_pmodes();

		if pmode_noun == 1 { mem[self.ip + 1] } else { mem[mem[self.ip + 1] as usize] }
	}

	fn get_verb(&self) -> i32 {
		let mem = &self.memory;
		let (_, pmode_verb, _) = self.get_pmodes();

		if pmode_verb == 1 { mem[self.ip + 2] } else { mem[mem[self.ip + 2] as usize] }
	}

	fn add(&mut self) -> OpcodeResult {
		let destination = self.memory[self.ip + 3] as usize;

		self.memory[destination] = self.get_noun() + self.get_verb();

		self.ip += 4;

		OpcodeResult::Ok
	}

	fn multiply(&mut self) -> OpcodeResult {
		let destination = self.memory[self.ip + 3] as usize;

		self.memory[destination] = self.get_noun() * self.get_verb();

		self.ip += 4;

		OpcodeResult::Ok
	}

	fn load_input(&mut self) -> OpcodeResult {
		let noun = self.memory[self.ip + 1];
		self.memory[noun as usize] = self.input;

		self.ip += 2;

		OpcodeResult::Ok
	}

	fn print_output(&mut self) -> OpcodeResult {
		let noun = self.memory[self.ip + 1];
		println!("{}", self.memory[noun as usize]);

		self.ip += 2;

		OpcodeResult::Ok
	}

	fn jump_if_true(&mut self) -> OpcodeResult {
		if self.get_noun() != 0 {
			self.ip = self.get_verb() as usize;
		} else {
			self.ip += 3;
		}

		OpcodeResult::Ok
	}

	fn jump_if_false(&mut self) -> OpcodeResult {
		if self.get_noun() == 0 {
			self.ip = self.get_verb() as usize;
		} else {
			self.ip += 3;
		}

		OpcodeResult::Ok
	}

	fn less_than(&mut self) -> OpcodeResult {
		let destination = self.memory[self.ip + 3] as usize;

		if self.get_noun() < self.get_verb() {
			self.memory[destination] = 1;
		} else {
			self.memory[destination] = 0;
		}

		self.ip += 4;

		OpcodeResult::Ok
	}

	fn equals(&mut self) -> OpcodeResult {
		let destination = self.memory[self.ip + 3] as usize;

		if self.get_noun() == self.get_verb() {
			self.memory[destination] = 1;
		} else {
			self.memory[destination] = 0;
		}

		self.ip += 4;

		OpcodeResult::Ok
	}
}
