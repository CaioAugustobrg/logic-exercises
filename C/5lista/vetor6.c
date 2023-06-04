#include <stdio.h>

int main() {
    int N;
    scanf("%d", &N);

    int A[N];
    int B[N];
    int C[N];

    // Ler os elementos do primeiro vetor (A)
    for (int i = 0; i < N; i++) {
        scanf("%d", &A[i]);
    }

    // Ler os elementos do segundo vetor (B)
    for (int i = 0; i < N; i++) {
        scanf("%d", &B[i]);
    }

    // Calcular a soma dos elementos e armazenar em C
    for (int i = 0; i < N; i++) {
        C[i] = A[i] + B[i];
    }

    // Imprimir a soma dos vetores C
    for (int i = 0; i < N; i++) {
        printf("%d ", C[i]);
    }

    return 0;
}
