#include <stdio.h>
#include <string.h>
#define max_var_length 100000

int main(int argc, char **argv){
	char* vars = argv[1];
	int pointer = 0;
	char title[10000];
	int tmp_pointer = 0;

	while (vars[pointer] != 0){
		int title_pointer = 0;
		char var_title[max_var_length];
		while (vars[pointer] != ':'){
			var_title[title_pointer++] = vars[pointer++];
		}
		var_title[title_pointer] = 0;pointer++;
		if (strcmp(var_title, "title")==0){
			tmp_pointer = 0;
			while (vars[pointer] != ';'){
				if (vars[pointer] == '\\'){pointer++;
					char to_print = '\\';
					switch(vars[pointer]){
						case	';'	:to_print=	';'	;break;
						case	'\\'	:to_print=	'\\'	;break;
						case	':'	:to_print=	':'	;break;
					}
					title[tmp_pointer++] = to_print;
				}
				else{title[tmp_pointer++] = vars[pointer];}
				pointer++;
			}
			title[tmp_pointer] = 0;
		}
		else{
			while (vars[pointer]!=0 && vars[pointer] != ';'){
				if (vars[pointer] == '\\'){pointer++;}pointer++;
			}
		}
		pointer++;
	}


	printf("\\documentclass{article}\n\\title{%s}\n\\usepackage{xcolor}\n\\usepackage{soul}\n\\usepackage{hyperref}\n\\begin{document}\n\t\\maketitle\n", title, title);
	pointer = 0;
	char* text = argv[2];
	char operation[100] = ""; // no operation will be longer than 100 chars.
	int operation_pointer = 0;
	int list_depth = 0;
	char current_link[8001] = ""; // it's (very) unlikely any link will be longer than this
	int current_link_pos = 0;
	int number_of_header_rows = 0;

	while (text[pointer] != 0){
                if (text[pointer] == '\n'){printf("<br>");}
                else if (text[pointer] == '&'){printf("\\&");}
                else if (text[pointer] == '%'){printf("\\%%");}
                else if (text[pointer] == '$'){printf("\\$");}
                else if (text[pointer] == '#'){printf("\\#");}
                else if (text[pointer] == '_'){printf("\\_");}
                else if (text[pointer] == '{'){printf("\\{");}
                else if (text[pointer] == '}'){printf("\\}");}
                else if (text[pointer] == '~'){printf("\\textasciitilde");}
                else if (text[pointer] == '^'){printf("\\textasciicircum");}
		else if (text[pointer] == '\\'){
			pointer++;
			if (text[pointer] != '\\'){
				while (text[pointer] != '\\'){
					operation[operation_pointer++] = text[pointer++];
				}
				operation[operation_pointer] = 0;
				operation_pointer = 0;
				if (strcmp("StartBold", operation) == 0){
					printf("\\textbf{");
				}
				if (strcmp("EndBold",operation)==0){
					printf("}");
				}
				else if (strcmp("StartItalic", operation) == 0){
                                        printf("\\textit{");
                                }
				else if (strcmp("EndItalic", operation) == 0){
					printf("}");
				}
				else if (strcmp("StartList", operation) == 0){
					printf("\n\\begin{itemize%d}", list_depth++);
				}
				else if (strcmp("EndList", operation) == 0){
					printf("\n\\end{itemize%d}", list_depth--);
				}
				else if (strcmp("StartListItem", operation) == 0){
					printf("\n\\item ");
				}
				else if (strcmp("EndListItem", operation) == 0){
					printf("");
				}
				else if (strcmp("StartLink", operation) == 0){
					pointer++;
					while (text[pointer] != '|'){
						current_link[current_link_pos++] = text[pointer++];
					}
					current_link[current_link_pos] = 0;
					current_link_pos = 0;
					printf("\\href{%s}{", current_link);
				}
				else if (strcmp("EndLink", operation) == 0){
					printf("}");
				}
				else if (strcmp("Section",operation) == 0){
					printf("\n\\section{Section}\n");
				}
				else if (strcmp("EndSection", operation) == 0){
					printf("");
				}
                                else if (strcmp("SubSection",operation) == 0){
                                        printf("\n\\subsection{SubSection}\n");
                                }
                                else if (strcmp("EndSubSection", operation) == 0){
                                        printf("");
                                }
				else if (strcmp("StartImage", operation) == 0){
					printf("\n\\begin{figure}\n\\includegraphics[width=\\linewidth]{");
				}
				else if (strcmp("EndImage", operation) == 0){
					printf("}\n\\end{figure}\n");
				}
				else if (strcmp("StartRight", operation) == 0){
					printf("\n\\begin{flushright}\n");
				}
				else if (strcmp("StartCenter", operation) == 0){
					printf("\n\\begin{center}\n");
				}
				else if (strcmp("EndRight", operation) == 0){
					printf("\n\\end{flushright}");
				}
				else if (strcmp("EndCenter", operation) == 0){
					printf("\n\\end{center}\n");
				}
				else if (strcmp("StartTable", operation) == 0){
					printf("<table>");
				}
				else if (strcmp("EndTable", operation) == 0){
					printf("</table>");
				}
				else if (strcmp("StartTableRow", operation) == 0){
					printf("<tr>");
				}
				else if (strcmp("EndTableRow", operation) == 0){
					printf("</tr>");
				}
				else if (strcmp("StartTableItem", operation) == 0){
					printf("<td>");
				}
				else if (strcmp("EndTableItem", operation) == 0){
					printf("</td>");
				}
				else if (strcmp("Startmark", operation) == 0){printf("\\hl{");}
				else if (strcmp("EndMark", operation) == 0){printf("}");}
				else if (strcmp("StartSuperscript", operation) == 0){printf("^{");}
				else if (strcmp("EndSuperscript", operation) == 0){printf("}");}
				else if (strcmp("StartSubscript", operation) == 0){printf("_{");}
				else if (strcmp("EndSubscript", operation) == 0){printf("}");}
				else if (strcmp("StartQuote", operation) == 0){printf("");}
				else if (strcmp("EndQuote", operation) == 0){printf("");}
				else if (strcmp("StartStrike", operation) == 0){printf("\\st{");}
				else if (strcmp("EndStrike", operation) == 0){printf("}");}
				else if (strcmp("Break", operation) == 0){printf("\n\\newpage\n");}
				else if (strcmp("StartSquareRoot", operation) == 0){printf("\\sqrt{");}
				else if (strcmp("EndSquareRoot", operation) == 0){printf("}");}
			}
			else{printf("\\textasciicircum");}
		}
		else{printf("%c", text[pointer]);}
		pointer++;

	}
	printf("\\end{document}");

}
