extern crate serialize;

use serialize::json::Json;

mod git;
mod env;

fn main() {
	let env_config: env::Environment = env::get("env.toml");	
	
	let repositories: Vec<Json> = git::repos(env_config.clone());

	for location in repositories.iter() {
		let url = String::from_str(location.find("ssh_url").unwrap().as_string().unwrap());	
		git::clone(url, env_config.workspace.clone())
	};
}