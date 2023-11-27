f, p = map(int, input().split())
winner_index = p
winner = ""

for _ in range(f):
    name, slice_index = input().split()
    slice_index = int(slice_index)
    winner_index -= slice_index
    
    if winner_index <= 0 and winner == '':
        winner = name

print(winner)
