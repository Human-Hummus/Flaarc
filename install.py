#!/usr/bin/env python
# I can't beleive this works.

import os
if os.system("if [[ `whoami` != \"root\" ]]\nthen\nexit 1\nfi") != 0:
    print("build error: root is required")
    exit(1);

# again, no clue how this works
if os.system("if [[ `pwd` == \"*/Flaarc\" ]]\nthen\nexit 1\nfi") != 0:
    print("please CD into the directory of Flaarc");
    exit(1);



print("building flaarc...");
if os.system("cargo build --release") != 0:
    print("Error during build phase")
    exit(1);

os.system("mkdir /lib/flaarc")

libfns = [
        ["if equal.c", "if equal"]
        ]

for i in libfns:
    os.system("cc \"`pwd`/standard_functions/" + i[0] + "\" -o \"/lib/flaarc/" + i[1] + "\"");
