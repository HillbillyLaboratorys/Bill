#include<stdio.h>

void shift(int pom){

    char key[20] = {"\0"};
    char msg[100] = {"\0"};
    char *ptr = key;
    printf("\nenter keyword under 20 charactors: ");
    scanf("%20s", key);
    printf("\nenter message under 100 charactors: ");
    fflush(stdin);
    scanf("%[^\n]100s", msg);
    //scanf("%100s", msg);

    int shif = 0;
    for (int j = 0; j < 200 && msg[j] != 0; j++){
        
        if (*ptr == '\0') { 
            ptr = key;
        }
        if(pom) {
            shif = msg[j] + (*ptr - 97);

            if (msg[j] != 32) {
                if (shif < 123) {
                    msg[j] = shif;
                }
                else {
                    msg[j] = shif - 26;
                }
            }
        }
        if(!pom) {
            shif = msg[j] - (*ptr - 97);

            if (msg[j] != 32) {
                if (shif > 96) {
                    msg[j] = shif;
                }
                else {
                    msg[j] = shif + 26;
                }
            }
        }
        ptr++;
    }

    printf("\n%s\n", msg);
}

int main() {

    char str[1];
    while (1) {
        printf("\nenter e for encript or d for decript: ");
        scanf("%1s", str);
     
        if (str[0] == 69 || str[0] == 101) {
            
            shift(1);
        }
        if (str[0] == 68 || str[0] == 100) {
            
            shift(0);
        }
    }
    

    return 0;
}