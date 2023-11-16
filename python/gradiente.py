def analisar_gradiente(texto):
    resultado = ''
    for caractere in texto:
        if caractere.isupper():
            resultado += 'U'  
        elif caractere.islower():
            resultado += 'L'  
        elif caractere.isspace():
            resultado += 'W'  
        elif caractere.isdigit():
            resultado += 'D'  
    return resultado


texto_entrada = input()


resultado_analise = analisar_gradiente(texto_entrada)
print(resultado_analise)
