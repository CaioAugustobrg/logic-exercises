#include <stdio.h>

int main () {
    int A,B,C;
    int resposta;
    scanf("%d %d %d", &A, &B, &C);
    scanf("%d", &resposta);
    if (A+B+C == resposta) {
        printf("Acertou\n");
        return 0;
    } 
   printf("Errou\n");
    return 0;
}