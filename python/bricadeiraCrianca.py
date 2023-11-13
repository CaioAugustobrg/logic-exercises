entrada = input()
vogais ="AEIOUaeiouÁÉÍÓÚáéíóúÂÊÔÎÛâêîôûÃÕãõ "
entradaSemVogal =""

for palavra in entrada:
    for letra in palavra:
        if letra  in vogais:
            entradaSemVogal = entradaSemVogal + letra
        else:
            entradaSemVogal = entradaSemVogal + "p"
print(entradaSemVogal)
        

