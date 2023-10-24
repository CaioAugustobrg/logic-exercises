import math;
soma = 0
qtdNumeros = 0
maiorNumero = 0
primeiroNumero = []
while True:
    n = int(input())
    if n != 0:
        qtdNumeros = qtdNumeros + 1
        soma = soma + n
        primeiroNumero.append(n)
    else:
        print(f'{qtdNumeros}')
        if len(primeiroNumero) > 0:
            print(f'{max(primeiroNumero)}')
        else:
            print(f'0')
        if soma == 0:
            print(f'0.00')
        else:
             print(f'{soma / qtdNumeros:.2f}') 
        break
# print(f'{qtdNumeros}')
# print(f'{maiorNumero}')
# print(f'{soma / qtdNumeros:.2f}')