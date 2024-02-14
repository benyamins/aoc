set shell := ["pwsh.exe", "-c"]
set dotenv-load
alias r := run-args
alias t := test

default:
	@just --list

run-args +ARGS:
	cargo run -- {{ ARGS }}

test TEST-NAME:
	cargo test {{ TEST-NAME }} -- --nocapture

get-input URL:
	curl.exe -H "Session: $env:SESSION" -o ./data/input.txt {{ URL }}
