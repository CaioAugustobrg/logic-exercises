A,B = input().split()
A = A.lower()
B = B.lower()
def cruzeiro(A,B):
    if A < B: 
        return 'B'
    else:
        return 'A'
resultado = cruzeiro(A,B)
print(resultado)