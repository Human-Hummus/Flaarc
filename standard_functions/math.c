#include "forall.c"
int main(int argc, char** argv){
	int x = -1;
	bool is_pow = false;
	while (TEXT[++x] != 0){
		char curchar = TEXT[x];
		if (isspace(curchar)){
			if (is_pow){print_direct("^");is_pow = false;}
		}
		if (curchar == '^'){
			is_pow = true;
		}


		if (curchar == '/'){
			print_direct("$div;");
		}
		else if (curchar == '*'){
			print_direct("$mul;");
		}
		else if (isalpha(curchar)){
			char to_print[] = {'/', '/', curchar, '/', '/', 0};
			print_direct(to_print);
		}
		else if (curchar == '['){
			print_direct("{sub:");
		}
		else if (curchar == ']'){
			print_direct("}");
		}
		
		else{	
			char to_print[] = {curchar, NULL};
			print_direct(to_print);
		}

	}
	if (is_pow){print_direct("^");}
}


