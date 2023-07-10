#include <stdio.h>

int main() {
    int num, soma_pares = 0, soma_impares = 0;
    
    for (;;) {
        scanf("%d", &num);
        if (num == 0) {
            break;
        }
        
        if (num % 2 == 0) {
            soma_pares += num;
        } else {
            soma_impares += num;
        }
    }
    
    printf("%d %d\n", soma_pares, soma_impares);
    
    return 0;
}