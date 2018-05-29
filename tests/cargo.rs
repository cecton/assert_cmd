extern crate assert_cmd;
extern crate predicates;

use std::process;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn main_binary() {
    let mut cmd = process::Command::main_binary().unwrap();
    cmd.env("stdout", "42");
    let expected: Vec<_> = vec![b'4', b'2', b'\n'];
    cmd.assert().success().stdout(&predicate::eq(expected));
}

#[test]
fn main_binary_with_empty_env() {
    let mut cmd = process::Command::main_binary().unwrap();
    cmd.env_clear().env("stdout", "42");
    let expected: Vec<_> = vec![b'4', b'2', b'\n'];
    cmd.assert().success().stdout(&predicate::eq(expected));
}

#[test]
fn cargo_binary() {
    let mut cmd = process::Command::cargo_bin("bin_fixture").unwrap();
    cmd.env("stdout", "42");
    let expected: Vec<_> = vec![b'4', b'2', b'\n'];
    cmd.assert().success().stdout(&predicate::eq(expected));
}

#[test]
fn cargo_binary_with_empty_env() {
    let mut cmd = process::Command::cargo_bin("bin_fixture").unwrap();
    cmd.env_clear().env("stdout", "42");
    let expected: Vec<_> = vec![b'4', b'2', b'\n'];
    cmd.assert().success().stdout(&predicate::eq(expected));
}

#[test]
fn cargo_example() {
    let mut cmd = process::Command::cargo_example("example_fixture").unwrap();
    cmd.env("stdout", "42");
    let expected: Vec<_> = vec![b'4', b'2', b'\n'];
    cmd.assert().success().stdout(&predicate::eq(expected));
}

#[test]
fn cargo_example_with_empty_env() {
    let mut cmd = process::Command::cargo_example("example_fixture").unwrap();
    cmd.env_clear().env("stdout", "42");
    let expected: Vec<_> = vec![b'4', b'2', b'\n'];
    cmd.assert().success().stdout(&predicate::eq(expected));
}
