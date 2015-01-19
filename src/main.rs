extern crate serialize;
extern crate alloc;
extern crate hyper;
extern crate url;

use serialize::json;

use hyper::client::Client;
use hyper::client::Response;
use hyper::header::Headers;
use hyper::header::shared::qitem;
use hyper::header::common::{Authorization, Accept, UserAgent};
use hyper::mime::Mime;

use url::Url;

use alloc::rc::Rc;

mod git;
mod env;

fn main() {
	let env_config: env::Environment = env::get("env.toml");	
	
	let mut headers = Headers::new();
	let mut client = Client::new();

	headers.set(UserAgent("octocopycat".to_string()));
	headers.set(Accept(vec![qitem("application/vnd.github.v3+json".parse().unwrap())]));
	headers.set(Authorization(env_config.github.token));
	
	let url = env_config.github.url.as_slice();
	
	let mut response: Response = match client.get(url).headers(headers).send() {
		Ok(r) => r,
		Err(msg) => panic!("Failed to connect: {:?}", msg)
	};
	
	let content = match response.read_to_string() {
		Ok(c) => c,
		Err(msg) => panic!("Failed to read content: {}", msg)
	};
	
	let json_body = match json::from_str(content.as_slice()) {
		Ok(j) => j,
		Err(msg) => panic!("Failed to parse JSON")
	};

	let repositories = json_body.as_array().unwrap();

	for location in repositories.iter() {
		let url = location.find("ssh_url").unwrap().to_string();
		git::clone(url, env_config.workspace.clone())
	};
}