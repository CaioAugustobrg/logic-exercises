#include <stdio.h>
//The input consists of two rows. The first row contains three integers A, B, and C 
//that represent the dimensions of the containers, while the second row contains another
// three integers X, Y, and Z that represent the dimensions of the ship.
int main() {
    int a, b, c, x, y, z;
    scanf("%d %d %d", &a, &b, &c);
    scanf("%d %d %d", &x, &y, &z);

    if (c > z) {
        printf("0\n");
    } else {
        int num_filas = x / a;
        int num_por_fila = y / b;
        printf("%d\n", num_filas * num_por_fila);
    }

    return 0;
}