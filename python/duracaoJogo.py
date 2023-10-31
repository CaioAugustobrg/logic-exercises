hora_inicial, minuto_inicial, hora_final, minuto_final = map(int, input().split())

duracao_minutos = (hora_final * 60 + minuto_final) - (hora_inicial * 60 + minuto_inicial)

if duracao_minutos < 0:
    duracao_minutos += 24 * 60

horas_duracao = duracao_minutos // 60
minutos_duracao = duracao_minutos % 60

if horas_duracao == 0 and minutos_duracao == 0:
    horas_duracao = 24  

print(f'O jogo durou {horas_duracao} hora(s) e {minutos_duracao} minuto(s).')
