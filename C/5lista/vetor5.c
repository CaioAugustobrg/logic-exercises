#include <stdio.h>

int main () {
    int n;
    scanf("%d", &n);
    int vetor[n];
 
    for (int i = 0; i < n; i++ ) {
        scanf("%d", &vetor[i]);
    }
    int pertence = 0;
       int x;
    scanf("%d", &x);
    for (int i = 0; i < n; i++) {
        if (vetor[i] == x) {
            pertence = 1;
            break;
        } 
    }
     if (pertence) {
            printf("pertence\n");
        } else {
            printf("nao pertence\n");
        }
        return 0;
}