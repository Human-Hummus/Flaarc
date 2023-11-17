#include <string.h>
#include <stdio.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdlib.h>

#define BIG_BUFFERLEN 10000000
#define SMALL_BUFFERLEN 1000
#define STYLE_FILE "/tmp/styledata"
#define STEP_MEM 1000 // number of bytes of memory to increase vecs by when filled
#define Vec struct vec

#define TEXT argv[1]

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


struct vec{
	int capacity;
	int length;
	char* content;
};

Vec new_vec(){
	Vec toret;
	toret.capacity = STEP_MEM;
	toret.length = 0;
	toret.content = malloc(STEP_MEM);
	return toret;
}

Vec append(Vec to_add_to, char to_add){
	if (to_add_to.capacity >= ++to_add_to.length){
		to_add_to.capacity+=STEP_MEM;
		to_add_to.content = realloc(to_add_to.content, to_add_to.capacity);
	}
	to_add_to.content[to_add_to.length] = to_add;
	return to_add_to;
}

char vecitem(Vec todo, int itemnum){
	if (itemnum > todo.length || itemnum < 0){
		printf("Fatal error: tried to access illegal vector location.");
		exit(1);
	}
	return todo.content[itemnum];
}

void dumpvec(Vec todo){
	free(todo.content);
	todo.length = 0;
	todo.capacity = 0;
}

Vec clear_vec(Vec vtd){
	vtd.length = 0;
	return vtd;
}

void print_direct(char* chars){
	int x = -1;
	while (chars[++x] != 0){
		putc(chars[x], stdout);
	}
}
