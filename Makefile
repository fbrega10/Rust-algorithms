make : 
	make help

make help:
	echo "Available options: \n 1 - build -->  $ cargo build\n 2 - test --> cargo test\n 3 - run --> cargo run"

make test:
	echo "Running tests..."
	cargo test

make build:
	echo "Building..."
	cargo build --release

make run:
	echo "Running: "
	cargo run
