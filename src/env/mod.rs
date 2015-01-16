extern crate toml;
extern crate std;

use self::toml::Value::{Table};

pub struct Environment {
	pub github: GitHub
}

pub struct GitHub {
	pub url: String,
	pub token: String
}

impl Environment {
	pub fn new(config: &str) -> Environment {
		get(config)
	}
}

fn get(config: &str) -> Environment {
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
	let config = Table(parser.parse().unwrap());

	Environment{ 
		github: GitHub{ 
			url: config.lookup("github.api.url").unwrap().to_string(),
			token: config.lookup("github.api.token").unwrap().to_string()
		} 
	}
}