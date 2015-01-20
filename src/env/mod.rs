extern crate toml;

use self::toml::Value::Table;
use std;

pub struct Environment {
	pub github: GitHub,
  pub workspace: String
}

// Using String vs. &str to 
// avoid lifetime issues - still learning.
pub struct GitHub {
	pub url: String,
	pub token: String
}

pub fn get(config: &str) -> Environment {
  let path = Path::new(config);
  let display = path.display();

  let mut file = match std::io::File::open(&path) {
      Err(why) => panic!("couldn't open {}: {}", display, why.desc),
      Ok(file) => file
  };

  let toml = match file.read_to_string() {
  	Err(why) => panic!("couldn't read {}: {}", display, why.desc),
  	Ok(t) => t
  };

  let mut parser = toml::Parser::new(toml.as_slice());
	let table = Table(parser.parse().unwrap());

	// look up toml::Value and unwrap it for the Option<&str> and unwrap it for the string.
	let url = table.lookup("github.api.url").unwrap().as_str().unwrap().to_string();
	let token = table.lookup("github.api.token").unwrap().as_str().unwrap().to_string();
  let workspace = table.lookup("workspace.path").unwrap().as_str().unwrap().to_string();

	Environment{ github: GitHub{ url: url, token: token }, workspace: workspace }
}