#define data 1000
#include <stdio.h>
#include <string.h>
#include <stdbool.h>

int main(int argc, char* argv[]){
	char arg1[data];
	char arg2[data];

	int tmp = 0;
	int curpoint = 0;
	while(argv[1][curpoint] == ' '){curpoint+=1;}
	while(argv[1][curpoint] != ' '){arg1[tmp++] = argv[1][curpoint];curpoint+=1;}
	arg1[tmp] = 0;
	tmp = 0;
	while(argv[1][curpoint] == ' '){curpoint+=1;}
	while(argv[1][curpoint] != ':'){arg2[tmp++] = argv[1][curpoint];curpoint+=1;}
        curpoint += 1;
	arg2[tmp] = 0;

	tmp = 0;
	while (arg1[tmp] == arg2[tmp] && arg1[tmp] != 0 && arg2[tmp] != 0){
		tmp+=1;
	}

	if  (arg1[tmp] != arg2[tmp]){
		return 0;
	}
	
	while (argv[1][curpoint] != EOF && argv[1][curpoint] != 0){
		putc(argv[1][curpoint++], stdout);
	}
}
