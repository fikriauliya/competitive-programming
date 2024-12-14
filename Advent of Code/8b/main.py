from collections import defaultdict

m = []
antennas = defaultdict(list)

while True:
    try:
        m.append(list(input()))
    except EOFError:
        break

for i in range(len(m)):
    for j in range(len(m[i])):
        if m[i][j] != ".":
            antennas[m[i][j]].append((i, j))

antinodes = set()


def inbound(y, x):
    return 0 <= y < len(m) and 0 <= x < len(m[0])


for k, v in antennas.items():
    for i in range(len(v)):
        antinodes.add(v[i])

        for j in range(i + 1, len(v)):
            delta = tuple(a - b for a, b in zip(v[j], v[i]))

            mult = 1
            while True:
                y, x = tuple(a - mult * b for a, b in zip(v[i], delta))
                mult += 1
                if not inbound(y, x):
                    break
                antinodes.add((y, x))

            mult = 1
            while True:
                y, x = tuple(a + mult * b for a, b in zip(v[j], delta))
                mult += 1
                if not inbound(y, x):
                    break
                antinodes.add((y, x))


print(len(antinodes))
