extern crate toml;
extern crate core;

use self::toml::Value::Table;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use self::core::str::Str;

#[derive(Clone)]
pub struct Environment {
	pub github: GitHub,
	pub workspace: String
}

#[derive(Clone)]
pub struct GitHub {
	pub url: String,
	pub token: String
}

pub fn get(config: &str) -> Environment {
	let path = Path::new(config);
	let display = path.display();

	let mut file = match File::open(&path) {
			Err(why) => panic!("couldn't open {}: {}", display, why),
			Ok(file) => file
	};

	let mut toml = String::new(); 
	match file.read_to_string(&mut toml) {
		Err(why) => panic!("couldn't read {}: {}", display, why),
		Ok(t) => t
	};

	let mut parser = toml::Parser::new(toml.as_slice());
	let table = match parser.parse() {
		Some(t) => Table(t),
		None => panic!("couldn't parse: {:?}", parser.errors)
	};

	let url = match table.lookup("github.api.url") {
		Some(url) => match url.as_str() {
			Some(url) => url.to_string(),
			None => "no-url".to_string()
		},
		None => "no-url".to_string()
	};

	let token = match table.lookup("github.api.token") {
		Some(token) => match token.as_str() {
			Some(token) => token.to_string(),
			None => "no-token".to_string()
		},
		None => "no-token".to_string()
	};

	let workspace = match table.lookup("workspace.path") {
		Some(workspace) => match workspace.as_str() {
			Some(workspace) => workspace.to_string(),
			None => "no-workspace".to_string()
		},
		None => "no-workspace".to_string()
	};

	println!("URL: {:?}, TOKEN: {:?}, WS: {:?}", url, token, workspace);

	Environment{ github: GitHub{ url: url, token: token }, workspace: workspace }
}
