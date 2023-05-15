#include <stdio.h>
#include <stdlib.h>

int main() {
    int xm, ym, xr, yr;
    scanf("%d %d %d %d", &xm, &ym, &xr, &yr);

    int dist = abs(xm - xr) + abs(ym - yr); // distância de Manhattan

    // arredonda a distância para o número mínimo de cruzamentos
    if (dist % 2 == 0) {
        printf("%d\n", dist / 2);
    } else {
        printf("%d\n", (dist / 2) + 1);
    }

    return 0;
}