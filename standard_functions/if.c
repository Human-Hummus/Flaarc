#include "forall.c"
int main(int argc, char** argv){
	int x = 0;
	char *tf = malloc(SMALL_BUFFERLEN);
	while (argv[1][x] != EOF && argv[1][x] != '|'){
		tf[x] = argv[1][x];++x;
	}
	tf[x++] = 0;
	tf = toLower(tf);
	if (iseq(tf, "yes") || iseq(tf, "true")){
		while (argv[1][x] != EOF){
			printf("%c", argv[1][x++]);
		}
	}
}
