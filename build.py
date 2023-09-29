#/bin/python3
import os
import subprocess
import sys
false = False
true = True

normal_cc = "clang"
fast_cc = "tcc"
normal_cflags = ["-Ofast", "-march=native", "-mtune=native", "-s"]
fast_cflags = ["-O0"]


docs = false
install = false
clean = false

cc = normal_cc
functions = [
    #input file     output files
    ["if.c",        ["if"]],
    ["if equal.c",  ["if equal"]],
    ["lower.c",     ["lower", "lowercase", "to lowercase"]],
    ["upper.c",     ["upper", "uppercase", "to uppercase"]],
    ["length.c",    ["length", "len"]],
    ]
outputs = [
    ["markdown.c", ["md", "MD", "markdown"]],
    ["html.c", ["html", "HTML", "web"]],
    ["plaintext.c", ["text", "plain text", "plaintext", "txt"]]
    ]

cflags = normal_cflags

x = 1
while x < len(sys.argv):
    a = sys.argv[x]
    if a == "install":
        install = true
    if a == "docs":
        docs == true
    if a == "fast":
        cc = fast_cc
        cflags = fast_cflags
    if a == "clean":
        clean = true
    x+=1;



if docs:
    print("building docs...")
    print("compiling readme.flaarc to README.md...")
    subprocess.run(["flaarc", '--input', 'readme.flaarc', '--output', 'README.md', '--format', "html"]);
    print("compiling readme.flaarc to src/help_info.txt")
    subprocess.run(["flaarc", '--input', 'readme.flaarc', '--output', 'src/help info.txt', "--format", "text"])

if install:
    if os.geteuid() != 0:
        print("You must be root")
        sys.exit(1)
    print("creating flaarc dir")
    subprocess.run(["mkdir", '-p', '/lib/flaarc/outputs'])

    print("building functions")
    for func in functions:
        ffile = "standard_functions/" + func[0]
        for outfile in func[1]:
            of = "/lib/flaarc/" + outfile
            print(f"building {of} from {ffile} with flags {cflags} with cc {cc}")
            subprocess.run([cc] + cflags + [ffile, '-o', of])
    print("building outputs")
    for op in outputs:
        ffile = "outputs/" + op[0]
        for outfile in op[1]:
            of = "/lib/flaarc/outputs/" + outfile
            print(f"building {of} from {ffile} with flags {cflags} with cc {cc}")
            subprocess.run([cc] + cflags + [ffile, '-o', of])
    print("building flaarc")
    if cc != normal_cc:
        subprocess.run(["cargo", "build"])
        subprocess.run(["cp", 'target/debug/flaarc', 'flaarc'])
    else:
        subprocess.run(["cargo", "build", '--release'])
        subprocess.run(["cp", 'target/release/flaarc', 'flaarc'])
    subprocess.run(["cp", 'flaarc', '/bin/flaarc'])

if clean:
    if os.geteuid() != 0:
        print("You must be root")
        sys.exit(1)
    print("removing target")
    subprocess.run(["rm", '-rf', 'target', 'flaarc'])
