n = int(input())
while True:
     tentativa = int(input())
     if (n > tentativa):
         print(f'O número correto é maior.')
     elif(n < tentativa):
         print(f'O número correto é menor.')
     else:
         print(f'Parabéns! Você acertou.')
         break