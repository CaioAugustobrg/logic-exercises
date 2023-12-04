n = int(input())

problemas = []
for i in range(n):
    p, s, d = input().split()
    problemas.append((s, int(d)))

problemas.sort(key=lambda x: (-x[1]))

solucoes_concatenadas = ''.join(p[0] for p in problemas)

print(solucoes_concatenadas)
