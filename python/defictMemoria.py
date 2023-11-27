def move_to_left(shelf, idx, steps):
    return max(0, idx - steps)

def move_to_right(shelf, idx, steps):
    return min(len(shelf) - 1, idx + steps)

def main():
    n = int(input()) 
    toys = input().split() 
    initial_shelf = toys[:]
    
    for _ in range(5):
        b, d, q = input().split()
        q = int(q)

        if d == 'E':
            idx = toys.index(b)
            new_idx = move_to_left(toys, idx, q)
        else:
            idx = toys.index(b)
            new_idx = move_to_right(toys, idx, q)

        toys.remove(b)
        toys.insert(new_idx, b)
    
    out_of_place = sum(1 for i in range(n) if initial_shelf[i] != toys[i])
    print(out_of_place)

if __name__ == "__main__":
    main()