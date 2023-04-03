flaarc:target/release/flaarc
	cp target/release/flaarc /bin/flaarc

target/release/flaarc: Cargo.toml src/*
	cargo build --release


clean:
	rm -R -d target
