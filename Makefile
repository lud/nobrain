build:
	cargo build --no-default-features --release

install:
	cargo install --path .

readme:
	iex util/generate-readme.exs