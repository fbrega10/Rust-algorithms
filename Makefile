help:
	echo "Available options: \n 1 - build -->  $ cargo build\n 2 - test --> cargo test\n 3 - run --> cargo run"

test:
	echo "Running tests..."
	cargo test

build:
	echo "Building..."
	cargo build --release

run:
	echo "Running: "
	cargo run

all:
	echo "building..."
	cargo build --release
	echo "Execution: \n"
	cargo run

fmt:
	cargo fmt
