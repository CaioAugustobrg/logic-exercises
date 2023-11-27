n = int(input())
i = 1
jogos = []

while i <= n:
    jogo = input()
    jogos.append(jogo)
    i += 1

jogos.reverse()
print(', '.join(jogos))
