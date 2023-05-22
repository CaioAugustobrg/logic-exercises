// Leia 1 valor inteiro N, que representa o número de casos de teste que vem a seguir.
//Cada caso de teste consiste de 3 valores reais, cada um deles com uma casa decimal.
//Apresente a média ponderada para cada um destes conjuntos de 3 valores, sendo que 
//o primeiro valor tem peso 2, o segundo valor tem peso 3 e o terceiro valor tem peso 5.
// Entrada
// O arquivo de entrada contém um valor inteiro N na primeira linha. Cada N linha a seguir
// contém um caso de teste com três valores com uma casa decimal cada valor.
// Saída
// Para cada caso de teste, imprima a média ponderada dos 3 valores, conforme exemplo abaixo.

#include <stdio.h>

int main () {
    int n;
    scanf("%d", &n);
    for (int i = 1; i <= n; i++) {
        float primeiro_valor, segundo_valor, terceiro_valor;
        scanf("%f %f %f", &primeiro_valor, &segundo_valor, &terceiro_valor);
        float media_ponderada = (primeiro_valor * 2 + segundo_valor * 3 + terceiro_valor * 5) / 10;
        printf("%.1f\n", media_ponderada);
    }
    return 0;
}