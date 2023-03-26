# Flaarc - (F)untional (La)tex-like M(ar)kdown Clone, With Some (C)-like Syntax. Also Lisp, Too.
##  What is it; Why should I use it?
I made this project because I was unhappy with Latex, as it's tough to write short documents in it, but I also didn't
want to use Markdown, becuase it'd lack many features present in Latex. Flaarc is supposed to have a syntax that
resembles normal text as little as possible, that way when you don't need the features, you don't have them, but when
you do, they're fairly easy to use. Does this mean you should use it? Probably not, at least not yet. As it is
*very* much **alpha quality**, I wouldn't recommend it for anything remotely serious. As a fun thing to try? Sure.


##  Using It From The CLI
The basic syntax is as follows:
(path to flaarc bin) -i (input file) -f (format) -o (output file)

Please note that while the format and output files aren't required, as they default to "markdown" and "out.md" respectively, the input file **is** required.

Some other options include:
- -help / -h:	print help info- 
	frog:		Don't worry about it.


##  Italics & Bold.
Italics can be defined with the text intended to be itallicized surounded by "//".
For example, "//italic text//" makes "*italic text*"

Bold is almost identical, but instead of surrounding the text with "//" you surround it with "\_\_"
For example, "\_\_bold text\_\_" becomes "**bold text**"

If you want to type \_\_ or // **without** bolding or italicizing the text, precede the chars with a backslash.
For example, to type \_\_, you would type \\\_\_ instead.


##  Hashes
Hashes are lines that begin with the '\#' symbol. There are a few types of hashes:


\#\#: this is a note, a line that starts with two hashes **and a space** will be discarded by the Flaarc parser.
\#section {name}: make a section, this means make a <h2> tag in the emmitted HTML.
\#title {title}: set the title, this will be the title of the webpage, and will be printed at the top, defaults to: "title". Creative, I know.
\#setfont {font} set the font.

to type a \# on it's own, ensure it's preceded by a backslash, like this: \\\#.



##  Features I want to add:
- mage support.- 
	standard library of functions.- 
	an optional GUI.- 
	better error messages.
