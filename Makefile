.PHONY: all clean

all:
	cargo build --release
	cp ./target/release/reversi ./reversi
clean:
	cargo clean
	rm reversi