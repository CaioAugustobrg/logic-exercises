#include <stdio.h>
// Leia um valor inteiro N. Este valor será a quantidade de valores que serão lidos em seguida.
// Para cada valor lido, mostre uma mensagem em inglês dizendo se este valor lido é par (EVEN),
// ímpar (ODD), positivo (POSITIVE) ou negativo (NEGATIVE). No caso do valor ser igual a zero (0),
// embora a descrição correta seja (EVEN NULL), pois por definição zero é par, seu programa deverá imprimir apenas NULL.
int main () {
    int n;
    printf("Digite a quantidade de casos teste: ");
    scanf("%d", &n);
    for (int i = 0; i < n; i++) {
        int x;
        printf("Digite o valor %d: ", i+1);
        scanf("%d", &x);

        if (x == 0) {
            printf("NULL");
        } else {
            if ((x % 2 == 0) && (x < 0)) {
                printf("EVEN NEGATIVE\n");
            } if ((x % 2 == 0) && (x > 0)) {
                printf("EVEN POSITIVE\n");
            } if ((x % 2 != 0) && (x > 0)) {
                printf("ODD POSITIVE\n");
            } if ((x % 2 != 0) && (x < 0)) {
                printf("ODD NEGATIVE\n");
            }
        }
    }
      return 0;
}