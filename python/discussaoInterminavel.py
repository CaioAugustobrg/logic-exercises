n = int(input())
i = 1
qtdP = 0
qtdU = 0
while i <= n:
    text = input().split('.')
    for letter in text[0]:
        if (letter == "P") or (letter == 'p'):
            qtdP = qtdP + 1
        if (letter == "u") or (letter == "U"):
            qtdU = qtdU + 1
    i = i + 1 
print(f'{qtdP} {qtdU}')
    
