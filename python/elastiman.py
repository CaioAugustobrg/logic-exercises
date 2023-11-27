def apply_gravity(matrix):
    n = len(matrix)

    for col in range(n):
        for row in range(n - 2, -1, -1):
            if matrix[row][col] == 'o':
                current_row = row
                while current_row < n - 1 and matrix[current_row + 1][col] == '.':
                  
                    matrix[current_row][col], matrix[current_row + 1][col] = matrix[current_row + 1][col], matrix[current_row][col]
                    current_row += 1

    return matrix

n = int(input())
scene = []
for _ in range(n):
    row = input().split()
    scene.append(row)

result = apply_gravity(scene)
for row in result:
    print(' '.join(row))
