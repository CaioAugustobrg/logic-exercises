#include <stdio.h>

int main () {
  int X, Y;

  scanf("%d%d", &X, &Y);
  if ((X <= 432 && X >= 0) && (Y >= 0 && Y <= 468)) {
    printf("dentro\n");
    return 0;
  }
  else {
    printf("fora\n");
    return 0;
  }
  return 0;
}