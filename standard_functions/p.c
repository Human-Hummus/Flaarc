#include <stdio.h>


int main(int argc, char* argv[]){
	int x = 0;
	while (argv[1][x] != EOF && argv[1][x] != 0){
		if (argv[1][x] == '\n'){
			printf(" ");
		}
		else{
			printf("%c", argv[1][x]);
		}
		x++;
	}

}
