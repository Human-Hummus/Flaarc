CCOMPILER = gcc -O3


flaarc:target/release/flaarc funcs
	cp target/release/flaarc /bin/flaarc

target/release/flaarc: Cargo.toml src/*
	cargo build --release
	

funcs:
	mkdir /lib/flaarc -p
	$(CCOMPILER) "standard_functions/if equal.c" -o "/lib/flaarc/if equal"
	$(CCOMPILER) "standard_functions/lower.c" -o "/lib/flaarc/lower"
	$(CCOMPILER) "standard_functions/upper.c" -o "/lib/flaarc/upper"
	$(CCOMPILER) "standard_functions/p.c" -o "/lib/flaarc/p"
	$(CCOMPILER) "standard_functions/length.c" -o "/lib/flaarc/length"


clean:
	rm -R -d target
