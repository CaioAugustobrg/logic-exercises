tempo = float(input())
velocidade = float(input())

distance = float(tempo * velocidade)
litros = float(distance / 14.2)  

print('{} {:.2f}'.format(distance,litros))