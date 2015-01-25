extern crate serialize;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use serialize::json::Json;

mod git;
mod env;

fn main() {
	let env_config: env::Environment = env::get("env.toml");

	let repositories: Vec<Json> = git::repos(env_config.clone());
	let mut messages: Vec<String> = Vec::new();
	let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

	for location in repositories.iter() {
		let url = String::from_str(location.find("ssh_url").unwrap().as_string().unwrap());
		let thread_tx = tx.clone();
		git::clone(url, env_config.workspace.clone(), thread_tx);
	};

	for threads in repositories.iter() {
		println!("{}", rx.recv().unwrap());
	}
}
