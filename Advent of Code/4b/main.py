m = []
while True:
    try:
        line = input()
        m.append(list(line))
    except EOFError:
        break

res = 0


def check(m, i, j):
    a, b, c, d = m[i - 1][j - 1], m[i + 1][j + 1], m[i - 1][j + 1], m[i + 1][j - 1]
    if m[i][j] == "A" and set(a + b) == set("MS") and set(c + d) == set("MS"):
        return True
    return False


dirs = [(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)]

for i in range(1, len(m) - 1):
    for j in range(1, len(m[i]) - 1):
        res += 1 if check(m, i, j) else 0

print(res)
