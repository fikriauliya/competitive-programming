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

for k, v in antennas.items():
    for i in range(len(v)):
        for j in range(i + 1, len(v)):
            delta = tuple(a - b for a, b in zip(v[j], v[i]))
            y, x = tuple(a - b for a, b in zip(v[i], delta))
            antinodes.add((y, x))
            y, x = tuple(a + b for a, b in zip(v[j], delta))
            antinodes.add((y, x))


# filter out out of bounds antinode
antinodes = list(
    filter(lambda x: 0 <= x[0] < len(m) and 0 <= x[1] < len(m[0]), antinodes)
)

print(len(antinodes))
