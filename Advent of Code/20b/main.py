from collections import defaultdict
import heapq

m = []
while True:
    try:
        line = input()
        m.append(list(line))
    except EOFError:
        break

start = None
end = None

for i in range(len(m)):
    for j in range(len(m[i])):
        if m[i][j] == "S":
            start = (i, j)
        elif m[i][j] == "E":
            end = (i, j)

q = []
heapq.heappush(q, (0, start))

dist = {}

while q:
    d, pos1 = heapq.heappop(q)

    dist[pos1] = d

    if pos1 == end:
        break

    deltas = [(-1, 0), (0, 1), (0, -1), (1, 0)]

    for delta in deltas:
        npos = (pos1[0] + delta[0], pos1[1] + delta[1])
        if (
            0 <= npos[0] < len(m)
            and 0 <= npos[1] < len(m[npos[0]])
            and m[npos[0]][npos[1]] != "#"
        ):
            if npos not in dist:
                heapq.heappush(q, (d + 1, npos))

for i in range(len(m)):
    for j in range(len(m[i])):
        if (i, j) in dist:
            print(f"{dist[(i, j)]:<3}", end="")
        else:
            print(f"{m[i][j]:<3}", end="")
    print()

savings = defaultdict(int)
total = 0

for pos1, d1 in dist.items():
    for pos2, d2 in dist.items():
        distance = abs(pos2[0] - pos1[0]) + abs(pos2[1] - pos1[1])
        if distance > 20:
            continue

        saving = d2 - (d1 + distance)
        if saving > 0:
            savings[saving] += 1
        if saving >= 100:
            total += 1

print(savings)
print(total)
