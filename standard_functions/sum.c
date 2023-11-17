#include "forall.c"
#define DBG printf("This is the debug thing\n");
#define OP -


char curnum[255], curnum_p = 0;
long long total = 0;
bool is_negative = false;

void convert_number_to_real_number(){
	unsigned int curmul = 1;
	curnum_p--;
	register signed long long toadd = 0;
	while (curnum_p > -1){
		toadd += curnum[curnum_p--]*curmul;
		curmul*=10;
	}
	if (is_negative){toadd*=-1;}
	if (toadd == 0) {return;}
	printf("toadd: %lld\n", toadd);
	total=total OP toadd;
	curnum_p = 0;
}

int main(int argc, char** argv){
	int x = 0, text_length = strlen(TEXT);

	while (x < text_length){
		if ((TEXT[x] & 0b11110000) == 0b00110000){
			curnum[curnum_p++] = TEXT[x] & 0b00001111;
		}
		else{
			convert_number_to_real_number();
			is_negative = false;
			if (TEXT[x] == '-'){
				is_negative=true;
			}
		}
		x+=1;

	}
	convert_number_to_real_number();
	printf("%lld\n", total);
}


