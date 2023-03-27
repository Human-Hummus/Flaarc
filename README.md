# Flaarc - (F)untional (La)tex-like M(ar)kdown Clone, With Some (C)-like Syntax. Also Lisp, Too.
##  What is it; Why should I use it?
I made this project because I was unhappy with Latex, as it's tough to write short documents in it, but I also didn't
want to use Markdown, because it'd lack many features present in Latex. Flaarc is supposed to have a syntax that
resembles normal text as little as possible, that way when you don't need the features, you don't have them, but when
you do, they're fairly easy to use. Does this mean you should use it? Probably not, at least not yet. As it is
*very* much **alpha quality**, I wouldn't recommend it for anything remotely serious. As a fun thing to try? Sure.

##  Using It From The CLI
The basic syntax is as follows:
(path to flaarc bin) -i (input file) -f (format) -o (output file)

Please note that while the format and output files aren't required, as they default to "markdown" and "out.md" respectively, the input file **is** required.

Some other options include:

- --help / -h:	print help info
- frog:		Don't worry about it.


##  Italics & Bold.
Italics can be defined with the text intended to be italicized surrounded by "//".
For example, "//italic text//" makes "*italic text*"

Bold is almost identical, but instead of surrounding the text with "//" you surround it with "\_\_"
For example, "\_\_bold text\_\_" becomes "**bold text**"

If you want to type \_\_ or // **without** bolding or italicizing the text, precede the chars with a backslash.
For example, to type \_\_, you would type \\\_\_ instead.


##  Hashes
Hashes are lines that begin with the '\#' symbol. There are a few types of hashes:


\#\#: this is a note, a line that starts with two hashes **and a space** will be discarded by the Flaarc parser.

\#section {name}: make a section, this means make a \<h2\> tag in the emmitted HTML.

\#title {title}: set the title, this will be the title of the webpage and will be printed at the top, defaults to: "title". Creative, I know.

\#setfont {font} set the font.

\#image {path\_to\_image}, include image in document.

to type a \# on its own, ensure it's preceded by a backslash, like this: \\\#.


##  Functions
Functions in Flaarc are just programs. You can make your own functions by copying the executable file into "/lib/flaarc".
The syntax of a function is as follows:
{ FUNCTION\_NAME\_HERE : FUNCTION ARG HERE }
Upon seeing this, the parser will try to run a program in the directory /lib/flaarc and will pass the "FUNCTION ARG HERE" section as **one** CLI argument to the program. Anything the program writes to the standard output will be parsed and then put in the document. Because it's parsed, it's syntactically legal to declare variables and even call other functions within the output of the program. 
Note: if the function is called without a ':', and therefore no argument to pass to the program, it will run the program without passing anything to it.

One way this could be used is if you want to be able to import your name, make a title, etc. **without** having to retype it every time you make a new document, you could make a function that you could call at the beginning of the document that returns the needed text.

There are reserved functions:

- The list function makes lists.
- The link function makes links.
##  Variables
To declare a variable, you need a "\#" preceded by a newline, immediately proceeded by the text "define", then a space, then the name of the variable, then a space, then the content of the variable, and finally, a newline.
For example, if you want to declare the variable "x" and set it to contain "this is in x!" you'd type:

\#define x this is in x!

To use a variable within text, type the variable's name preceded by a '$', and proceded by a space. Note that the space will be ignored, if you want a space after a variable, type two spaces.
To access the contents of x, we could say:

I've set the variable x to "$x ".
This will be converted to:
I've set the variable x to "this is in x!".

Note how the space following $x is present *before* parsing, but not after.



##  Thank you.
I want to say thank you for showing interest in this hobby project. It's been fun for me, and I hope you'll have fun too.



##  Features I want to add:

- web image support.
- standard library of functions.
- an optional GUI.
- better error messages.
- highlight support.
- table support.
