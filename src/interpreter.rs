use std::fs::File;
use std::io::{self, Read};

enum States {
	Empty,
	Ready
}
pub struct Interpreter {
	code: Vec<u8>,
	code_ptr: usize,
	tape: Vec<u8>,
	tape_ptr: usize,
	state: States
}

impl Interpreter {
	pub fn new() -> Interpreter {
		let mut tape = Vec::<u8>::with_capacity(30000);
		for i in 0..30000 {
			tape.push(0);
		}
		Interpreter {
			code: Vec::<u8>::new(),
			code_ptr: 0,
			tape,
			tape_ptr: 0,
			state: States::Empty
		}
	}

	pub fn read_code(&mut self, file_name: &str) {
		let mut file = File::open(file_name).expect("file not found");
		let mut buffer = String::new();
		file.read_to_string(&mut buffer);

		self.code = buffer.into_bytes();
		self.state = States::Ready;
	}

	pub fn run(&mut self){
		println!("Code length: {}", self.code.len());
		println!("Tape length: {}", self.tape.len());
		println!("Output:\n");



		while self.code_ptr < self.tape.len() {

			if self.state == States::Empty {break};

			if self.code.len() == self.code_ptr {break};

			match self.code[self.code_ptr] as char {
				'>' => self.tape_ptr += 1,
				'<' =>  if self.tape_ptr == 0 {
							panic!("Moved out of tape");
						} else {
							self.tape_ptr -= 1
						},
				'+' =>  if self.tape[self.tape_ptr] == 255 {
							self.tape[self.tape_ptr] = 0;
						} else {
							self.tape[self.tape_ptr] += 1;
						},
				'-' =>  if self.tape[self.tape_ptr] == 0 {
							self.tape[self.tape_ptr] = 255;
						} else {
							self.tape[self.tape_ptr] -= 1;
						},
				'.' =>  print!("{}", self.tape[self.tape_ptr]as char),
				/*',' =>  {
							let mut input_text = String::new();
							io::stdin()
								.read_line(&mut input_text)
								.expect("failed to read from stdin");

							let input = input_text.trim().to_string().into_bytes();
							self.tape[self.tape_ptr] = input;
						},*/
				'[' =>  {
							if self.tape[self.tape_ptr] == 0 {
								let mut opened = 0;
								self.code_ptr += 1;
								while self.code_ptr < self.code.len() {
									if self.code[self.code_ptr] as char == ']' && opened == 0 {
										break;
									} else if self.code[self.code_ptr] as char == '[' {
										opened += 1;
									} else if self.code[self.code_ptr] as char == ']' {
										opened -= 1;
									}
									self.code_ptr += 1;
								}
							}

						},
				']' =>  {
							if self.tape[self.tape_ptr] != 0 {
								let mut closed = 0;
								self.code_ptr -= 1;
								while self.code_ptr >= 0 {
									if self.code[self.code_ptr] as char == '[' && closed == 0 {
										break;
									} else if self.code[self.code_ptr] as char == ']' {
										closed += 1;
									} else if self.code[self.code_ptr] as char == '[' {
										closed -= 1;
									}
									self.code_ptr -= 1;
								}
							}

						},
				_ => ()

			}
			self.code_ptr += 1;
		}
		println!("\n--------------Done--------------")
	}

}