#[allow(non_snake_case)]
extern crate clap;
use clap::{Arg, App};

mod interpreter;
use interpreter::*;

fn main() {
    let matches = App::new("Brainfuck Interpreter in Rust")
	    .version("1.0")
	    .author("Patrick Michl <batzi1234@googlemail.com>")
	    .about("Interpret Brainfuck files")
	    .arg(Arg::with_name("file")
		    .help("Specify the input file")
		    .required(true)
		    .index(1))
	    .get_matches();

	let filename = matches.value_of("file").unwrap();

	let mut fuck_me = Interpreter::new();
	fuck_me.read_code(filename);
	fuck_me.run();
}
