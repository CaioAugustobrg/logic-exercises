N = int(input())
maior_soma = float('-inf')
menor_soma = float('inf')
soma_total = 0

for _ in range(N):
    X, Y = map(int, input().split())
    soma = 0
    
    if X % 2 == 0:
        X += 1
    
    for _ in range(Y):
        soma += X
        X += 2
    
    print(soma)
    
    maior_soma = max(maior_soma, soma)
    menor_soma = min(menor_soma, soma)
    soma_total += soma


media = (maior_soma + menor_soma) / 2

print(f'{maior_soma}')
print(f'{menor_soma}')
print(f'{media:.2f}')
