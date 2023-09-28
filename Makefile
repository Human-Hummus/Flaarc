CFLAGS=-Ofast -march=native -mtune=native -s

qinstall:qfuncs qflaarc

qflaarc:
	cargo build
	cp target/debug/flaarc /bin/flaarc

qfuncs:
	mkdir /lib/flaarc -p
	tcc "standard_functions/if equal.c" -o "/lib/flaarc/if equal"
	tcc "standard_functions/lower.c" -o "/lib/flaarc/lower"
	tcc "standard_functions/upper.c" -o "/lib/flaarc/upper"
	tcc "standard_functions/p.c" -o "/lib/flaarc/p"
	tcc "standard_functions/length.c" -o "/lib/flaarc/length"
	mkdir -p /lib/flaarc/outputs
	tcc "outputs/markdown.c" -o "/lib/flaarc/outputs/markdown" 
	tcc "outputs/markdown.c" -o "/lib/flaarc/outputs/md"
	tcc "outputs/html.c" -o "/lib/flaarc/outputs/html" 
	tcc "outputs/html.c" -o "/lib/flaarc/outputs/HTML"
	tcc "outputs/plaintext.c" -o "/lib/flaarc/outputs/plaintext"
	tcc "outputs/plaintext.c" -o "/lib/flaarc/outputs/text"
	tcc "outputs/plaintext.c" -o "/lib/flaarc/outputs/txt"



install:flaarc

flaarc: funcs
	cargo build --release
	cp target/release/flaarc /bin/flaarc
	@echo "If you get an error following this, you might need to add /bin to your PATH. To do this, run \"export PATH=\$$PATH:/bin\""

funcs:
	mkdir /lib/flaarc -p
	$(CC) $(CFLAGS) "standard_functions/if equal.c" -o "/lib/flaarc/if equal"
	$(CC) $(CFLAGS) "standard_functions/lower.c" -o "/lib/flaarc/lower"
	$(CC) $(CFLAGS) "standard_functions/upper.c" -o "/lib/flaarc/upper"
	$(CC) $(CFLAGS) "standard_functions/p.c" -o "/lib/flaarc/p"
	$(CC) $(CFLAGS) "standard_functions/length.c" -o "/lib/flaarc/length"
	mkdir -p /lib/flaarc/outputs
	$(CC) $(CFLAGS) "outputs/markdown.c" -o "/lib/flaarc/outputs/markdown" 
	$(CC) $(CFLAGS) "outputs/markdown.c" -o "/lib/flaarc/outputs/md"
	$(CC) $(CFLAGS) "outputs/html.c" -o "/lib/flaarc/outputs/html"
	$(CC) $(CFLAGS) "outputs/html.c" -o "/lib/flaarc/outputs/HTML"
	$(CC) $(CFLAGS) "outputs/plaintext.c" -o "/lib/flaarc/outputs/plaintext"
	$(CC) $(CFLAGS) "outputs/plaintext.c" -o "/lib/flaarc/outputs/text"
	$(CC) $(CFLAGS) "outputs/plaintext.c" -o "/lib/flaarc/outputs/txt"


clean:
	rm -R -d target outputs/pdfoutput/target


docs: 
	flaarc -i readme.flaarc -o README.md -f markdown
	flaarc -i readme.flaarc -o "src/help info.txt" -f text
