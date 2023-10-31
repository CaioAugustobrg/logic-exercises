cargo = input()
tempo_servico = int(input())
salario_atual = float(input())

salario_minimo = 1039.00

if salario_atual < salario_minimo:
    print("Salário inválido!")
else:
    if cargo == "Gerente":
        if tempo_servico <= 3:
            reajuste = salario_atual * 0.12
        elif tempo_servico <= 6:
            reajuste = salario_atual * 0.13
        else:
            reajuste = salario_atual * 0.15
    elif cargo == "Engenheiro":
        if tempo_servico <= 3:
            reajuste = salario_atual * 0.07
        elif tempo_servico <= 6:
            reajuste = salario_atual * 0.11
        else:
            reajuste = salario_atual * 0.14
    else:
        reajuste = salario_atual * 0.05

    salario_reajustado = salario_atual + reajuste

    print(f"{reajuste:.2f}")
    print(f"{salario_reajustado:.2f}")
