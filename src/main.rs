extern crate hyper;
extern crate serialize;

use serialize::{json};
use std::str::FromStr;
use std::io::process::{Command, ProcessOutput};
use std::comm;
use std::thread::Thread;
use hyper::client::Client;
use hyper::header::{Headers, Authorization, Accept, UserAgent};
use hyper::mime::{Mime};


fn clone(url: &str, tx: Sender<&str>) {
	let workspace = Path::new("{env.workspace");
	match Command::new("git").cwd(&workspace).arg("clone").arg(url).output() {
		Err(msg) => panic!("Failed to run {}", msg.desc),
		Ok(ProcessOutput { error: err, output: out, status: exit }) => {
      if exit.success() {
      		let s = String::from_utf8_lossy(out.as_slice());
      		let msg = format!("Cloned {} successfully!\n{}", url, s);
          tx.send(msg.as_slice());
      } else {
          let s = String::from_utf8_lossy(err.as_slice());
          let msg = format!("Failed to clone: {}\n{}\n\n", url, s);
          tx.send(msg.as_slice());
      }
  	},
	};
}

fn main() {

	let url = "{env.github.api.url}";
	let token = "token {env.github.api.token}".to_string();
	let agent = "octocopycat".to_string();
	let mime: Mime = FromStr::from_str("application/vnd.github.v3+json").unwrap();
	let mut headers = Headers::new();
	let mut client = Client::new();

	headers.set(Accept(vec![mime]));
	headers.set(Authorization(token));
	headers.set(UserAgent(agent));

	let mut response = match client.get(url).headers(headers).send() {
		Ok(response) => response,
		Err(msg) => panic!("There was an error getting url: {}", msg)
	};

	let content = match response.read_to_string() {
		Ok(content) => content,
		Err(msg) => panic!("There was a problem reading response {}", msg)
	};

	let json_body = match json::from_str(content.as_slice()) {
		Ok(json_body) => json_body,
		Err(msg) => panic!("There was problem parsing json {}", msg)
	};		

	let repositories = json_body.as_array().unwrap();
	let (tx, rx): (Sender<&str>, Receiver<&str>) = comm::channel();

	for location in repositories.iter() {
		let url = location.find("ssh_url").unwrap().clone();
		let thread_tx = tx.clone();
		Thread::spawn(move || {
			clone(url.as_string().unwrap(), thread_tx)
		}).detach();
	};

	for message in rx.iter() {
		println!("Message: {}", message)
	};
}