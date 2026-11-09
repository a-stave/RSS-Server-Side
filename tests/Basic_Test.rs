use std::process::Command;

#[test]
fn test_hello_world() {
    let output = Command::new(env!("CARGO_BIN_EXE_rss-server-side"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    assert_eq!(stdout.trim(), "Hello,world!");
}
