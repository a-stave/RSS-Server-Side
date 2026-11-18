use std::process::Command;


#[test]
#[ignore = "Example only: server process runs indefinitely so this test will hang"]
fn test_main_function() {

    // Integration tests often invoke main and check for IO. Longer to run but more realistic.
    let output = Command::new(env!("CARGO_BIN_EXE_rss-server-side"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    assert_eq!(stdout.trim(), "Listening on http://127.0.0.1:3000");
}