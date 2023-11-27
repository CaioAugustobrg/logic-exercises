p = 0
def mover(n, origem, aux, destino):
    global p
    if n > 0 and p > 0:
        mover(n - 1, origem, destino, aux)  
        if p > 0:     
            p = p - 1
            disk = origem[0].pop() 
            destino[0].append(disk)        
        mover(n - 1, aux, origem, destino)
h, p = [int(x) for x in input().split()]
pino_original = (list(range(h, 0, -1)), "origem")
pino_auxiliar = ([], "auxiliar")
pino_destino = ([], "destino")
mover(len(pino_original[0]), pino_original, pino_auxiliar, pino_destino)
print(len(pino_original[0]), len(pino_auxiliar[0]), len(pino_destino[0]))