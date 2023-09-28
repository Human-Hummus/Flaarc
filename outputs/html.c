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


	printf("<!DOCTYPE html><html><style>table, th, td {border: 1px solid black; border-collapse: collapse;padding:5px;}</style><head><title>%s</title></head><body><h1>%s</h1>", title, title);
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
                else if (text[pointer] == '\''){printf("&apos;");}
                else if (text[pointer] == '"'){printf("&quot;");}
                else if (text[pointer] == '<'){printf("&lt;");}
                else if (text[pointer] == '>'){printf("&gt;");}
                else if (text[pointer] == '&'){printf("&amp;");}
		else if (text[pointer] == '\\'){
			pointer++;
			if (text[pointer] != '\\'){
				while (text[pointer] != '\\'){
					operation[operation_pointer++] = text[pointer++];
				}
				operation[operation_pointer] = 0;
				operation_pointer = 0;
				if (strcmp("StartBold", operation) == 0){
					printf("<strong>");
				}
				if (strcmp("EndBold",operation)==0){
					printf("</strong>");
				}
				else if (strcmp("StartItalic", operation) == 0){
                                        printf("<em>");
                                }
				else if (strcmp("EndItalic", operation) == 0){
					printf("</em>");
				}
				else if (strcmp("StartList", operation) == 0){
					printf("<ul>");
				}
				else if (strcmp("EndList", operation) == 0){
					printf("</ul>");
				}
				else if (strcmp("StartListItem", operation) == 0){
					printf("<li>");
				}
				else if (strcmp("EndListItem", operation) == 0){
					printf("</li>");
				}
				else if (strcmp("StartLink", operation) == 0){
					while (text[pointer] != '|'){
						current_link[current_link_pos++] = text[pointer++];
					}
					current_link[current_link_pos] = 0;
					current_link_pos = 0;
					printf("<a href=\"%s\">", current_link);
				}
				else if (strcmp("EndLink", operation) == 0){
					printf("</a>");
				}
				else if (strcmp("Section",operation) == 0){
					printf("<h2>");
				}
				else if (strcmp("EndSection", operation) == 0){
					printf("</h2>");
				}
                                else if (strcmp("SubSection",operation) == 0){
                                        printf("<h3>");
                                }
                                else if (strcmp("EndSubSection", operation) == 0){
                                        printf("</h3>");
                                }
				else if (strcmp("StartImage", operation) == 0){
					printf("<img src=\"");
				}
				else if (strcmp("EndImage", operation) == 0){
					printf("\">");
				}
				else if (strcmp("StartRight", operation) == 0){
					printf("<div style=\"text-align: right\">");
				}
				else if (strcmp("StartCenter", operation) == 0){
					printf("<div style=\"text-align: right\">");
				}
				else if (strcmp("EndRight", operation) == 0 || strcmp("EndCenter", operation) == 0){
					printf("</div>");
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
				else if (strcmp("Startmark", operation) == 0){printf("<mark>");}
				else if (strcmp("EndMark", operation) == 0){printf("</mark>");}
				else if (strcmp("StartSuperscript", operation) == 0){printf("<sup>");}
				else if (strcmp("EndSuperscript", operation) == 0){printf("</sup>");}
				else if (strcmp("StartSubscript", operation) == 0){printf("<sub>");}
				else if (strcmp("EndSubscript", operation) == 0){printf("</sub>");}
				else if (strcmp("StartQuote", operation) == 0){printf("<blockquote>");}
				else if (strcmp("EndQuote", operation) == 0){printf("</blockquote>");}
				else if (strcmp("StartStrike", operation) == 0){printf("<del>");}
				else if (strcmp("EndStrike", operation) == 0){printf("</del>");}
				else if (strcmp("Break", operation) == 0){printf("<div style=\"page-break-before:always\"</div>");}
			}
			else{printf("\\\\");}
		}
		else{printf("%c", text[pointer]);}
		pointer++;

	}
	printf("</body></html>");

}
