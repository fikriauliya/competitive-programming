m = []
while True:
    try:
        line = input()
        m.append(list(line))
    except EOFError:
        break

res = 0


def rec(m, i, j, word, dir):
    if len(word) == 0:
        return True

    if i < 0 or j < 0 or i >= len(m) or j >= len(m[i]):
        return False

    if m[i][j] == word[0]:
        return rec(m, i + dir[0], j + dir[1], word[1:], dir)

    return False


dirs = [(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)]

for i in range(len(m)):
    for j in range(len(m[i])):
        word = "XMAS"
        for dir in dirs:
            res += 1 if rec(m, i, j, word, dir) else 0

print(res)
