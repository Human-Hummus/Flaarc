#include <stdio.h>
#include <string.h>
#define max_var_length 100000

int main(int argc, char **argv){
	char* vars = argv[1];
	int pointer = 0;
	while (vars[pointer] != 0){
		int title_pointer = 0;
		char var_title[max_var_length];
		while (vars[pointer] != ':'){
			var_title[title_pointer++] = vars[pointer++];
		}
		var_title[title_pointer] = 0;pointer++;
		if (strcmp(var_title, "title")==0){
			while (vars[pointer] != ';'){
				if (vars[pointer] == '\\'){pointer++;
					char to_print = '\\';
					switch(vars[pointer]){
						case	';'	:to_print=	';'	;break;
						case	'\\'	:to_print=	'\\'	;break;
						case	':'	:to_print=	':'	;break;
					}
					printf("%c",to_print);
				}
				else{printf("%c",vars[pointer]);}
				pointer++;
			}
			printf("\n");
		}
		else{
			while (vars[pointer]!=0 && vars[pointer] != ';'){
				if (vars[pointer] == '\\'){pointer++;}pointer++;
			}
		}
		pointer++;
	}
	pointer = 0;
	char* text = argv[2];
	char operation[100] = ""; // no operation will be longer than 100 chars.
	int operation_pointer = 0;
	int list_depth = 0;
	char current_link[8001] = ""; // it's (very) unlikely any link will be longer than this
	int current_link_pos = 0;
	int number_of_header_rows = 0;

	while (text[pointer] != 0){
		if (text[pointer] == '\\'){
			pointer++;
			if (text[pointer] != '\\'){
				while (text[pointer] != '\\'){
					operation[operation_pointer++] = text[pointer++];
				}
				operation[operation_pointer] = 0;
				operation_pointer = 0;
				if (strcmp("StartBold", operation) == 0 || strcmp("EndBold",operation) == 0){
					printf("**");
				}
				else if (strcmp("StartItalic", operation) == 0 || strcmp("EndItalic",operation) == 0){
                                        printf("//");
                                }
				else if (strcmp("StartList", operation) == 0){
					list_depth++;
				}
				else if (strcmp("EndList", operation) == 0){
					list_depth--;
				}
				else if (strcmp("StartListItem", operation) == 0){
					printf("\n");
					int depth_printed = 0;
					while (depth_printed++<list_depth){
						printf("\t");
					}
					printf("- ");
				}
				else if (strcmp("EndListItem", operation) == 0){} //do nothing
				else if (strcmp("StartLink", operation) == 0){
					while (text[pointer] != '|'){
						current_link[current_link_pos++] = text[pointer++];
					}
					current_link[current_link_pos] = 0;
					current_link_pos = 0;
					printf("[");
				}
				else if (strcmp("EndLink", operation) == 0){
					printf("](%s)",current_link);
				}
				else if (strcmp("Section",operation) == 0){
					printf("## ");
				}
				else if (strcmp("EndSection", operation) == 0){
					printf("\n");
				}
				else if (strcmp("StartImage", operation) == 0){
					printf("![image](");
				}
				else if (strcmp("EndImage", operation) == 0){
					printf(")");
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
				else if (strcmp("StartTable", operation) == 0){number_of_header_rows=0;}
				else if (strcmp("EndTable", operation) == 0){}
				else if (strcmp("StartTableRow", operation) == 0){
					if (number_of_header_rows > 0){
						printf("\n|");
						while (number_of_header_rows-- > 0){
							printf("---|");
						}
						number_of_header_rows=-1;
					}
					printf("\n|");
				}
				else if (strcmp("EndTableRow", operation) == 0){}//nothing
				else if (strcmp("StartTableItem", operation) == 0){}//nothing
				else if (strcmp("EndTableItem", operation) == 0){
					if (number_of_header_rows>-1){number_of_header_rows++;}
					printf("|");
				}
				else if (strcmp("Startmark", operation) == 0){printf("<mark>");}
				else if (strcmp("EndMark", operation) == 0){printf("</mark>");}
				else if (strcmp("StartSuperscript", operation) == 0){printf("<sup>");}
				else if (strcmp("EndSuperscript", operation) == 0){printf("</sup>");}
				else if (strcmp("StartSubscript", operation) == 0){printf("<sub>");}
				else if (strcmp("EndSubscript", operation) == 0){printf("</sub>");}
				else if (strcmp("StartQuote", operation) == 0){printf("\n>");}
				else if (strcmp("EndQuote", operation) == 0){printf("\n");}
				else if (strcmp("StartStrike", operation) == 0){printf("~~");}
				else if (strcmp("EndStrike", operation) == 0){printf("~~");}
			}
			else{printf("\\\\");}
		}
		else if (text[pointer] == '_'){printf("\\_");}
		else if (text[pointer] == '*'){printf("\\*");}
		else if (text[pointer] == '#'){printf("\\#");}
		else if (text[pointer] == '<'){printf("\\<");}
		else if (text[pointer] == '>'){printf("\\>");}
		else{printf("%c", text[pointer]);}
		pointer++;

	}

}
