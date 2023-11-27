entrada = input().capitalize()
uppercase_next = False
saida = ''

for letra in entrada:
    if letra == '_':
        uppercase_next = True
    elif uppercase_next:
        saida += letra.upper()
        uppercase_next = False
    else:
        saida += letra

print(saida)
