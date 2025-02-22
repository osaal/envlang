use std::process::Command;

#[test]
fn invalid_file_extension() {
    let output: std::process::Output = Command::new(env!("CARGO_BIN_EXE_envlang"))
        .arg("tests/data/io_invalidextension.txt")
        .output()
        .expect("Failed to run envlang");

    assert_eq!(
        output.status.code(),
        Some(101), // Note that Rust always sets exit code 101 on panics, so this is not IO-specific
        "Expected exit code 101, got {:?}", output.status.code()
    );
}

#[test]
fn no_arguments() {
    let output: std::process::Output = Command::new(env!("CARGO_BIN_EXE_envlang"))
        .output()
        .expect("Failed to run envlang");

    assert_eq!(
        output.status.code(),
        Some(1),
        "Expected exit code 1, got {:?}", output.status.code()
    );
}

#[test]
fn too_many_arguments() {
    let output: std::process::Output = Command::new(env!("CARGO_BIN_EXE_envlang"))
        .arg("tests/data/io_validextension.envl")
        .arg("something_else")
        .output()
        .expect("Failed to run envlang");

    assert_eq!(
        output.status.code(),
        Some(2),
        "Expected exit code 2, got {:?}", output.status.code()
    );
}