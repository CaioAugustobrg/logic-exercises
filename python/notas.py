n = int(input())
i = 0
aprovados = 0
mediaAlunos = 0
while i < n:
    n1,n2,n3,faltas = input().split()
    faltas = int(faltas)
    n1 = float(n1)
    n2 = float(n2)
    n3 = float(n3)
    media = (n1 + n2 + n3) / 3
    mediaArredondada = media
    if (media >= 5) and (faltas <= 16):
        print(f'{mediaArredondada:.1f} Aprovado')
        aprovados = aprovados + 1
    elif (3 < mediaArredondada < 5) and (faltas <= 16):
        print(f'{mediaArredondada:.1f} Exame') 
    else:
        print(f'{mediaArredondada:.1f} Reprovado')
    mediaAlunos = mediaAlunos + mediaArredondada
    i = i + 1    
print(f'{mediaAlunos / n :.1f} {aprovados}')