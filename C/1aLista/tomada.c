#include <stdio.h>
// A invenção do carro tornou muito mais rápido e mais barato realizar viagens de longa distância.
// Realizar uma viagem rodoviária tem dois tipos de custos: cada quilômetro percorrido na rodovia
// tem um custo associado (não só devido ao consumo de combustível mas também devido ao
// desgaste das peças do carro, pneus, etc.), mas também é necessário passar por vários pedágios
// localizados ao longo da rodovia.
// Os pedágios são igualmente espaçados ao logo da rodovia; o começo da estrada não possui
// um pedágio, mas o seu final pode estar logo após um pedágio (por exemplo, se a distância entre
// dois pedágios consecutivos for de 37 km e a estrada tiver 111 km, o motorista deve pagar um
// pedágio aos 37 km, aos 74 km e aos 111 km, logo antes de terminar a sua viagem)
int main (){
int T1, T2, T3, T4;
  T1 = 2 <= T1 <= 6;
  T2 = 2 <= T2 <= 6;
  T3 = 2 <= T3 <= 6;
  T4 = 2 <= T4 <= 6;
 scanf("%d %d %d %d",&T1,&T2,&T3,&T4);
 printf("%d\n", T1 + T2 + T3 + T4 - 3 );
return 0;
};