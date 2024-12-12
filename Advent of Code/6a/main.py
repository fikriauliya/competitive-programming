import sys

sys.setrecursionlimit(10**6)

m = []
while True:
    try:
        m.append(list(input()))
    except EOFError:
        break

x, y = 0, 0
for i in range(len(m)):
    for j in range(len(m[i])):
        if m[i][j] in (">", "<", "^", "v"):
            y, x = i, j
            break

visited = set()


def rec(y, x, dir):
    if y < 0 or y >= len(m) or x < 0 or x >= len(m[y]):
        return

    visited.add((y, x))

    ny = y + dir[0]
    nx = x + dir[1]

    if ny < 0 or ny >= len(m) or nx < 0 or nx >= len(m[ny]):
        return

    if m[ny][nx] == "#":
        next_dirs = {
            (0, 1): (1, 0),
            (0, -1): (-1, 0),
            (1, 0): (0, -1),
            (-1, 0): (0, 1),
        }
        ny = y + next_dirs[dir][0]
        nx = x + next_dirs[dir][1]
        dir = next_dirs[dir]
    rec(ny, nx, dir)


dirs = {">": (0, 1), "<": (0, -1), "^": (-1, 0), "v": (1, 0)}
rec(y, x, dirs[m[y][x]])

print(len(visited))
