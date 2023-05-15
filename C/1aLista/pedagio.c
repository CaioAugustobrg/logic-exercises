#include <stdio.h>

int main (){
  int L,D,P,K;
 
  scanf("%d %d", &L,&D);
  scanf("%d %d", &P, &K);
  printf("%d\n", ((L/D) * P) + (L * K));
  return 0;
};
// A entrada consiste de duas linhas. A primeira linha da entrada contém dois inteiros L e D (1 ≤ L,
// D ≤ 104), indicando o comprimento da estrada e a distância entre pedágios, respectivamente. A
// segunda linha contém dois inteiros K e P (1 ≤ K, P ≤ 10 4 ), indicando o custo por quilômetro
// percorrido e o valor de cada pedágio. O primeiro pedágio está localizado no quilômetro D da
// estrada (ou seja, a distância do início da estrada para o primeiro pedágio é D quilômetros).
// L = COMPRIMENTO DA ESTRADA
// D = DISTÂNCIA ENTRE PEDÁGIOS
// K = CUSTO POR QUILÔMETRO
// P = VALOR DE CADA PEDÁGIO