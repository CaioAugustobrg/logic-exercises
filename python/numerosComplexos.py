import math

x1,y1 = input().split(" ")
x2,y2 = input().split(" ")
x1 = float(x1)
y1 = float(y1)
x2 = float(x2)
y2 = float(y2)
distanceBetweenThem = ((math.pow(x2 - x1, 2)) + (math.pow(y2 - y1,2))) ** (1/2)

terceiroValor = complex(input())
somaComlex = ((math.pow(terceiroValor.real, 2)) 
 + (math.pow(terceiroValor.imag, 2))) ** (1/2)

print('{:.4f}\n{:.4f}'.format(distanceBetweenThem, somaComlex))