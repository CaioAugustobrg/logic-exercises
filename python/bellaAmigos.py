c = int(input())
i = 1
cuidado = 0
while i <= c:
    nome = str(input())
    if nome == "":
        break
    if nome.upper() == "ANDRÉ":
        cuidado = cuidado + 1
    i = i + 1
if cuidado == 0:
    print('Seguro!')
else:
    print("Cuidado!")
        