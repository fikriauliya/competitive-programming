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


def in_bound(pos):
    return pos[0] >= 0 and pos[0] < len(m) and pos[1] >= 0 and pos[1] < len(m[0])


def add(a, b):
    return (a[0] + b[0], a[1] + b[1])


def is_empty(pos, positions):
    if not in_bound(pos):
        return True

    return pos not in positions


def side(positions):
    res = 0
    for pos in positions:
        corners = [(-1, -1), (-1, 1), (1, -1), (1, 1)]
        for corner in corners:
            sides = [(corner[0], 0), (0, corner[1])]
            c = add(pos, corner)
            s = [add(pos, side) for side in sides]

            if (
                not is_empty(s[0], positions)
                and not is_empty(s[1], positions)
                and is_empty(c, positions)
            ):
                res += 1
                continue

            if is_empty(s[0], positions) and is_empty(s[1], positions):
                res += 1
                continue

    return res


total = 0
for i in range(len(m)):
    for j in range(len(m[0])):
        paths = path(visited, m, i, j, m[i][j])
        if len(paths) == 0:
            continue
        v = volume(paths)
        s = side(paths)
        total += v * s

print(total)
