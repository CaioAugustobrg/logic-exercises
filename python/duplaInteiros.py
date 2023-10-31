T = int(input())
i = 1
while i <= T:
    A,N = input().split()
    A = int(A)
    N = int(N)
    soma = 0
    for j in range(N-1) :
        print(A, end= ' ')
        soma+=A
        A+=1
    print(A)
    soma+=A
    print(soma)
    i+=1
