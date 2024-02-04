set shell := ["pwsh.exe", "-c"]
alias r := run-args

default:
	@just --list

run-args +ARGS:
	cargo run -- {{ ARGS }}
