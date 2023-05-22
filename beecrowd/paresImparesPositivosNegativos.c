#include <stdio.h>


// Leia 5 valores Inteiros. A seguir mostre quantos valores digitados foram pares,
// quantos valores digitados foram ímpares, quantos valores digitados foram positivos
// e quantos valores digitados foram negativos.
// Entrada
// O arquivo de entrada contém 5 valores inteiros quaisquer.
// Saída
// Imprima a mensagem conforme o exemplo fornecido, uma mensagem por linha,
// não esquecendo o final de linha após cada uma.

int main () {
    int n;
    int pares = 0;
    int impares = 0;
    int positivos = 0;
    int negativos = 0;
    for (int i = 1; i <= 5; i++) {
        scanf("%d", &n);
        if (n % 2 == 0 && (n >= 0 || n <= 0)) {
            pares++;
        } if (n % 2 != 0 && (n >= 0 || n <= 0)) {
            impares++;
        } if (n > 0 && (n % 2 == 0 || n % 2 != 0)) {
            positivos++;
        } if (n < 0 && (n % 2 == 0 || n % 2 != 0)) {
        negativos++;
    }
    }
    printf("%d valor(es) par(es)\n", pares);
    printf("%d valor(es) impar(es)\n", impares);
    printf("%d valor(es) positivo(s)\n", positivos);
    printf("%d valor(es) negativo(s)\n", negativos);
    return 0;
}