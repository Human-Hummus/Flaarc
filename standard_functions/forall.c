#include <string.h>
#include <stdio.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdlib.h>
#define BIG_BUFFERLEN 10000000
#define SMALL_BUFFERLEN 1000
#define STYLE_FILE "/tmp/styledata"

char* toLower(char* s) {
  for(char *p=s; *p; p++) *p=tolower(*p);
  return s;
}

bool iseq(char* a, char* b){
	while (true){
		if (a[0] != b[0]){
			return false;
		}
		if (a[0] == 0){
			return true;
		}
		a++;b++;
	}
}
