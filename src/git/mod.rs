extern crate hyper;
extern crate serialize;

use self::hyper::client::Client;
use self::hyper::client::Response;
use self::hyper::header::{Headers, qitem, Authorization, Accept, UserAgent};
use self::hyper::mime::Mime;
use std::io::process::{Command, ProcessOutput};
use std::fmt;
use std::sync::mpsc::Sender;
use self::serialize::json;
use env::Environment;
use std::thread::Thread;

pub fn clone(url: String, workspace: String, tx: Sender<String>) {

	let workspace = Path::new(workspace);

  Thread::spawn(move || {
    match Command::new("git").cwd(&workspace).arg("clone").arg(url).output() {
      Err(msg) => tx.send("Failed to run.".to_string()).unwrap(),
      Ok(ProcessOutput { error: err, output: out, status: exit }) => {
        if exit.success() {
            tx.send(format!("Successful {}.", String::from_utf8_lossy(out.as_slice())).to_string());
        } else {
            tx.send(format!("Failed \n{}\n\n", String::from_utf8_lossy(err.as_slice())).to_string());
        }
      },
    };
  });
	
}

pub fn repos(env_config: Environment) -> Vec<json::Json> {
  
  let mut headers = Headers::new();
  let mut client = Client::new();

  headers.set(UserAgent("octocopycat".to_string()));
  headers.set(Accept(vec![qitem("application/vnd.github.v3+json".parse().unwrap())]));
  headers.set(Authorization(env_config.github.token));
  
  let url = env_config.github.url.as_slice();
  
  let mut response: Response = match client.get(url).headers(headers).send() {
    Ok(r) => {
      println!("Successfully retrieved repos for {}.", url);
      r
    },
    Err(msg) => panic!("Failed to connect: {:?}, {:?}", msg, url)
  };
  
  let content = match response.read_to_string() {
    Ok(c) => c,
    Err(msg) => panic!("Failed to read content: {}", msg)
  };
  
  let json_body = match json::from_str(content.as_slice()) {
    Ok(j) => j,
    Err(msg) => panic!("Failed to parse JSON")
  };

  json_body.as_array().unwrap().clone()
}
