#!/usr/bin/env bats

@test "no arguments" {
  result="$(../target/x86_64-unknown-linux-musl/release/homemade_yes|head -n 1)"
  [[ "$result" == "y" ]]
}

@test "arguments" {
  result="$(../target/x86_64-unknown-linux-musl/release/homemade_yes abc|head -n 1)"
  [[ "$result" == "abc" ]]
}

@test "--help" {
  result="$(../target/x86_64-unknown-linux-musl/release/homemade_yes --help|head -n 1)"
  [[ "$result" =~ "Outputs the string provided in the arguments forever" ]]
}

@test "--version" {
  result="$(../target/x86_64-unknown-linux-musl/release/homemade_yes --version|head -n 1)"
  [[ "$result" =~ "Mojavesoft implementation" ]]
}

@test "-V" {
  result="$(../target/x86_64-unknown-linux-musl/release/homemade_yes -V|head -n 1)"
  [[ "$result" =~ "Mojavesoft implementation" ]]
}

@test "-h" {
  result="$(../target/x86_64-unknown-linux-musl/release/homemade_yes -h|head -n 1)"
  [[ "$result" =~ "Outputs the string provided in the arguments forever" ]]
}

@test "-- -h" {
  result="$(../target/x86_64-unknown-linux-musl/release/homemade_yes -- -h|head -n 1)"
  [[ "$result" == "-h" ]]
}

@test "-- -V" {
  result="$(../target/x86_64-unknown-linux-musl/release/homemade_yes -- -V|head -n 1)"
  [[ "$result" == "-V" ]]
}

@test "-- --help" {
  result="$(../target/x86_64-unknown-linux-musl/release/homemade_yes -- --help|head -n 1)"
  [[ "$result" == "--help" ]]
}

@test "-- --version" {
  result="$(../target/x86_64-unknown-linux-musl/release/homemade_yes -- --version|head -n 1)"
  [[ "$result" == "--version" ]]
}
