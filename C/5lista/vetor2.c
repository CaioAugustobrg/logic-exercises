#include <stdio.h>

int main() {
    int n;
    //int menor_valor;
    scanf("%d", &n);
    int vetor[n];
    for (int i = 0; i < n; i++) {
    scanf("%d", &vetor[i]);
    }

    int menor = vetor[0];
    int indice_menor;
    
    for (int i = 1; i < n; i++) {
        if (vetor[i] < menor) {
            menor = vetor[i];
            indice_menor = i;
        }
    }
    printf("%d\n", indice_menor);
    return 0;
    }