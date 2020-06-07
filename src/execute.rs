use std::io::Write;
use std::process::{Command, Stdio};

static INTERPRETER: &str = "/usr/bin/osascript";

pub fn execute(_id: &str, _alter: bool) {
    let script_content = include_bytes!(env!("SCRIPT_PATH"));
    let mut executor = Command::new(INTERPRETER)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to spawn");
    let stdin = executor.stdin.as_mut().expect("Failed to fetch STDIN");
    stdin
        .write(script_content)
        .expect("Failed to write to STDIN");
}
