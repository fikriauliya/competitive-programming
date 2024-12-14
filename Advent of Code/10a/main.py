m = []
while True:
    try:
        chs = list(map(int, list(input())))
        m.append(chs)
    except EOFError:
        break


def count(y, x, num, tops):
    if y < 0 or y >= len(m) or x < 0 or x >= len(m[y]):
        return 0

    if m[y][x] != num:
        return 0

    if m[y][x] == 9 and (y, x) not in tops:
        tops.add((y, x))
        return 1

    dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)]
    total = 0
    for dir in dirs:
        total += count(y + dir[0], x + dir[1], num + 1, tops)
    return total


res = 0
for i in range(len(m)):
    for j in range(len(m[i])):
        tops = set()
        res += count(i, j, 0, tops)

print(res)
