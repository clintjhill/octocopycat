extern crate hyper;

use hyper::client::Client;
use hyper::client::Response;
use hyper::header::Headers;
use hyper::header::shared::qitem;
use hyper::header::common::{Authorization, Accept, UserAgent};
use hyper::mime::Mime;

//mod git;
mod env;

fn main() {
	let env_config: env::Environment = env::Environment::new("env.toml");	
	
	let mut headers = Headers::new();
	let mut client = Client::new();

	headers.set(UserAgent("octocopycat".to_string()));
	headers.set(Accept(vec![qitem("application/vnd.github.v3+json".parse().unwrap())]));

	// Runtime error: Places "" around value
	//headers.set(Authorization(env_config.github.token));
	// Works
	headers.set(Authorization("token a2b5d215c51f0bcd1ea2428ea840f0d50e336f80".to_string()));
	
	println!("{}", headers);
	
	// Runtime error: HttpUriError(RelativeUrlWithoutBase)
	//let url: &str = env_config.github.url.as_slice();
	// Works
	let url_slice = "https://api.github.com/orgs/aaa-ncnu-ie/repos";

	let mut response: Response = match client.get(url_slice).headers(headers).send() {
		Ok(r) => r,
		Err(msg) => panic!("Failed to connect: {:?} .. {}", msg, env_config.github.url)
	};
	
	match response.read_to_string() {
		Ok(r) => println!("Response: {}", r),
		Err(msg) => panic!("Error: {}", msg)
	}

	/*
	
	let json_body = try!(json::from_str(content.as_slice()));

	let repositories = json_body.as_array().unwrap();
	let (tx, rx): (Sender<&str>, Receiver<&str>) = comm::channel();

	for location in repositories.iter() {
		let url = location.find("ssh_url").unwrap().clone();
		let thread_tx = tx.clone();
		Thread::spawn(move || {
			git::clone(url.as_string().unwrap(), thread_tx)
		}).detach();
	};

	for message in rx.iter() {
		println!("Message: {}", message)
	};
	*/
}