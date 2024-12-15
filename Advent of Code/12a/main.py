m = []
while True:
    try:
        chs = list(input())
        m.append(chs)
    except EOFError:
        break

visited = [[False for _ in range(len(m[0]))] for _ in range(len(m))]


def path(visited, m, i, j, letter):
    if (
        i < 0
        or i >= len(m)
        or j < 0
        or j >= len(m[0])
        or visited[i][j]
        or m[i][j] != letter
    ):
        return []

    visited[i][j] = True
    paths = [(i, j)]
    paths += path(visited, m, i + 1, j, letter)
    paths += path(visited, m, i - 1, j, letter)
    paths += path(visited, m, i, j + 1, letter)
    paths += path(visited, m, i, j - 1, letter)
    return paths


def volume(paths):
    return len(paths)


def perimeter(paths):
    p = 0
    letter = m[paths[0][0]][paths[0][1]]
    for i in range(len(paths)):
        y, x = paths[i]

        if y - 1 < 0 or m[y - 1][x] != letter:
            p += 1
        if y + 1 >= len(m) or m[y + 1][x] != letter:
            p += 1
        if x - 1 < 0 or m[y][x - 1] != letter:
            p += 1
        if x + 1 >= len(m[0]) or m[y][x + 1] != letter:
            p += 1

    return p


total = 0
for i in range(len(m)):
    for j in range(len(m[0])):
        paths = path(visited, m, i, j, m[i][j])
        if len(paths) == 0:
            continue
        v = volume(paths)
        p = perimeter(paths)
        total += v * p

print(total)
