#![allow(non_snake_case)]
#[macro_use] extern crate lalrpop_util;
#[cfg_attr(test, macro_use)] extern crate paste;

mod parser;

pub fn compile(inString: String) -> String {
	let _parsed = parser::parse(inString);
	// TODO: what type does an empty table contain?
	String::from("output test")
}
