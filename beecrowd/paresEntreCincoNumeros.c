#include <stdio.h>

int main () {
    int n;
    int valores_pares;
    for (int i = 1; i <=5; i++) {
        scanf("%d", &n);
        if (n % 2 == 0) {
            valores_pares++;
        }
    }
    printf("%d Valores pares\n", valores_pares);
    return 0;
}