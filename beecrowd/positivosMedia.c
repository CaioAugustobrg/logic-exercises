#include <stdio.h>

int main() {
    int valores_positivos = 0;
    float soma_valores_positivos = 0;

    for (int i = 1; i <= 6; i++) {
        float n;
        scanf("%f", &n);
        
        if (n > 0) {
            valores_positivos++;
            soma_valores_positivos += n;
        }
    }

    float media = soma_valores_positivos / valores_positivos;
    printf("%d valores positivos\n", valores_positivos);
    printf("%.1f\n", media);

    return 0;
}