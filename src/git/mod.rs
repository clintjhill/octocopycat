use std::io::process::{Command, ProcessOutput};
use std::comm;

pub fn clone(url: &str, tx: Sender<&str>) {
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