// Faça um programa que leia 6 valores. Estes valores serão somente negativos ou positivos (desconsidere os valores nulos). A seguir, mostre a quantidade de valores positivos digitados.
// Entrada

// Seis valores, negativos e/ou positivos.
// Saída

// Imprima uma mensagem dizendo quantos valores positivos foram lidos.

#include <stdio.h>

int main () {
     int valoresLidos = 6;
    int contadorValoresPositivos = 0;
    int i;
    int valor;
    for ( int i = 1; i <= valoresLidos; i++) {
        scanf("%d", &valor);
        if (valor > 0) {
            contadorValoresPositivos++;
            
        }
    }
     printf("Foram digitados %d valores positivos.\n", contadorValoresPositivos);

    return 0;
}