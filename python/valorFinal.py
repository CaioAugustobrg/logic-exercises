valor_compra, num_parcelas = map(float, input().split())

if num_parcelas == 1:
    valor_final = valor_compra - (valor_compra * 0.05)
elif num_parcelas == 2:
    valor_final = valor_compra
elif num_parcelas == 3:
    valor_final = valor_compra + (valor_compra * 0.05)
elif num_parcelas == 4:
    valor_final = valor_compra + (valor_compra * 0.10)

valor_parcela = valor_final / num_parcelas

print(f"{valor_final:.2f} {valor_parcela:.2f}")
