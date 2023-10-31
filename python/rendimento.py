valorDeposito = float(input())
valorPorcentagem = float(input())
valorSomaPorcentagem = (valorDeposito * valorPorcentagem) / 100
valorTotal = valorSomaPorcentagem + valorDeposito
print('{:.2f}\n{:.2f}'.format(valorSomaPorcentagem, valorTotal))