#include <stdio.h>

int main () {
    int n,soma = 0;

    for (;;) {
        scanf("%d",&n);

        if (n == 0) {
            break;
        }

        if (n % 2 == 0) {
            soma += n;
        }
    }
    printf("%d\n",soma);
}