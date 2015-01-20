use std::io::process::{Command, ProcessOutput};
use std::fmt;

pub fn clone(url: String, workspace: String) {

	let workspace = Path::new(workspace);

	match Command::new("git").cwd(&workspace).arg("clone").arg(url).output() {
		Err(msg) => panic!("Failed to run {}", msg.desc),
		Ok(ProcessOutput { error: err, output: out, status: exit }) => {
      if exit.success() {
      		let s = String::from_utf8_lossy(out.as_slice());
      		println!("{}\n", s);
      } else {
          let s = String::from_utf8_lossy(err.as_slice());
          println!("Failed to clone: {}\n\n", s);
      }
  	},
	};
}