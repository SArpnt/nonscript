#![allow(non_snake_case)]

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
	let inPath = Path::new("./tests/testCode.non");
	let mut inFile = match File::open(&inPath) {
		Err(why) => panic!("couldn't open {}: {}", inPath.display(), why),
		Ok(inFile) => inFile,
	};
	let mut inString = String::new();
	match inFile.read_to_string(&mut inString) {
		Err(why) => panic!("couldn't read {}: {}", inPath.display(), why),
		Ok(_) => print!("{} contains:\n{}\n", inPath.display(), inString),
	}
	let inputString = inString;


	let outputString = "testOutput";

	let outPath = Path::new("./output.c");
	let mut outFile = match File::create(&outPath) {
		Err(why) => panic!("couldn't create {}: {}", outPath.display(), why),
		Ok(outFile) => outFile,
	};
	match outFile.write_all(outputString.as_bytes()) {
		Err(why) => panic!("couldn't write to {}: {}", outPath.display(), why),
		Ok(_) => println!("successfully wrote to {}", outPath.display()),
	}
}
