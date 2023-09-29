#include "forall.c"


int main(int argc, char* argv[]){
	int x = 0;
	while (argv[1][x] != 0){
		printf("%c", tolower(argv[1][x++]));
	}
}
