#![allow(non_snake_case)]
#[macro_use] extern crate lalrpop_util;
#[macro_use] extern crate paste;

mod parser;

pub fn compile(inString: String) -> String {
	let parsed = parser::parse(inString);
	// TODO: what type does an empty table contain?
	String::from("output test")
}
