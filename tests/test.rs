extern crate assert_cli;

#[test]
fn main_thread() {
    assert_cli::Assert::main_binary()
        .stdin("hello")
        .stdout()
        .is("hello")
        .unwrap();
}

#[test]
fn child_thread() {
    assert_cli::Assert::main_binary()
        .with_args(&["--child"])
        .stdin("hello")
        .stdout()
        .is("hello")
        .unwrap();
}

#[test]
fn subchild_thread() {
    assert_cli::Assert::main_binary()
        .with_args(&["--subchild"])
        .stdin("hello")
        .stdout()
        .is("hello")
        .unwrap();
}
