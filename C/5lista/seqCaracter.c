#include <stdio.h>

int main () {
    char palavra[16];
    scanf("%15s", palavra);
    int i;
    for (i = 0; palavra[i] != '\0'; i++);
    printf("%d\n", i);
    return 0;
}