#!/usr/bin/env bats

@test "no arguments" {
  result="$(../target/x86_64-unknown-linux-musl/release/posix-only|head -n 1)"
  [ "$result" == "y" ]
}

@test "arguments" {
  result="$(../target/x86_64-unknown-linux-musl/release/homemade_yes abc|head -n 1)"
  [ "$result" == "abc" ]
}

@test "--help" {
  result="$(../target/x86_64-unknown-linux-musl/release/posix-only --help|head -n 1)"
  [ "$result" == "--help" ]
}

@test "--version" {
  result="$(../target/x86_64-unknown-linux-musl/release/posix-only --version|head -n 1)"
  [ "$result" == "--version" ]
}

@test "-h" {
  result="$(../target/x86_64-unknown-linux-musl/release/posix-only -h|head -n 1)"
  [ "$result" == "-h" ]
}

@test "-V" {
  result="$(../target/x86_64-unknown-linux-musl/release/posix-only -V|head -n 1)"
  [ "$result" == "-V" ]
}
