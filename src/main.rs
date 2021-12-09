#![allow(non_snake_case)]

use std::fs;
use std::io::prelude::*;
use std::path::Path;

fn compile(inString: String) -> String {
	// TODO: what type does an empty table contain?
	String::from("output test")
}

fn main() {
	let inDirPath = fs::read_dir("./tests/").unwrap();
	let outDirPath = fs::read_dir("./testOutputs/").unwrap();
	let mut compiledFiles = 0;

	for path in inDirPath {
		let inFilePath = path.unwrap().path();
		println!("found file: {}", inFilePath.display());
		if let Some(stem) = inFilePath.extension() {
			if stem.to_str().unwrap() != "non" {
				continue;
			}
		} else {
			continue;
		}
		println!("compiling file: {}", inFilePath.display());
		compiledFiles += 1;
		let mut inFile = match fs::File::open(&inFilePath) {
			Err(why) => panic!("couldn't open {}: {}", inFilePath.display(), why),
			Ok(inFile) => inFile,
		};
		let mut inString = String::new();
		match inFile.read_to_string(&mut inString) {
			Err(why) => panic!("couldn't read {}: {}", inFilePath.display(), why),
			Ok(_) => (),
		}

		let outputString = compile(inString);

		let outFilePathStr = format!("./testOutputs/{}.c", inFilePath.file_stem().unwrap().to_str().unwrap());
		let outFilePath = Path::new(&outFilePathStr);
		let mut outFile = match fs::File::create(&outFilePath) {
			Err(why) => panic!("couldn't create {}: {}", outFilePath.display(), why),
			Ok(outFile) => outFile,
		};
		match outFile.write_all(outputString.as_bytes()) {
			Err(why) => panic!("couldn't write to {}: {}", outFilePath.display(), why),
			Ok(_) => println!("successfully wrote to {}", outFilePath.display()),
		}
	}
	println!("finished compiling {} files", compiledFiles)
}
