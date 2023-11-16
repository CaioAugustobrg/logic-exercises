sopa_entregue = input()
sopa_abandonada = input()
sopa_entregue = sopa_entregue.upper()
sopa_abandonada =  sopa_abandonada.upper()

def contar_letras(sopa_entregue, sopa_abandonada):
    comidas = sum(letra in sopa_abandonada and letra not in sopa_entregue for letra in 'JOHN')
    deixadas = sum(letra in sopa_entregue and letra not in sopa_abandonada for letra in 'JOHN')
    return comidas, deixadas

# Entrada das strings de letras da sopa entregue e da sopa abandonada

# Chamada da função e impressão do resultado
comidas, deixadas = contar_letras(sopa_entregue, sopa_abandonada)
print(f'{comidas} {deixadas}')
