extern crate hyper;
extern crate serialize;

use serialize::{json};
use std::str::FromStr;
use std::io::process::{Command, ProcessOutput};
use std::thread::Thread;

use hyper::client::Client;
use hyper::client::Response;
use hyper::header::Headers;
use hyper::header::shared::qitem;
use hyper::header::common::{Authorization, Accept, UserAgent};
use hyper::mime::Mime;
use hyper::mime::TopLevel::{Application};

//mod git;

fn main() {

	let url = "{env.github.api.url}";
	let token = String::from_str("token {env.github.api.token}");
	let agent = String::from_str("octocopycat");
	//let mime: Mime = FromStr::from_str("application/vnd.github.v3+json").unwrap();
	let mut headers = Headers::new();
	let mut client = Client::new();

	headers.set(Accept(vec![
		qitem(Mime(Application, "vnd.github.v3+json", vec![]))
		]));

	headers.set(Authorization(token));
	headers.set(UserAgent(agent));

	let mut response: Response = match client.get(url).headers(headers).send() {
		Err(msg) => panic!("Failed to connect: {}", msg),
		Ok(r) => r
	};

	/*
	let content = try!(response.read_to_string());
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