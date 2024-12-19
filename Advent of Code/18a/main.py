import heapq

# SIZE = 7
SIZE = 71
# FIRST = 12
FIRST = 1024


def draw():
    for i in range(SIZE):
        for j in range(SIZE):
            print(m[i][j], end="")
        print()


m = [["." for _ in range(SIZE)] for _ in range(SIZE)]

ctr = 0
while True:
    try:
        x, y = map(int, input().split(","))
        m[y][x] = "#"
        ctr += 1

        if ctr == FIRST:
            break

    except EOFError:
        break

draw()

# bfs
pq = []

st = (0, 0)
end = (SIZE - 1, SIZE - 1)

heapq.heappush(pq, (0, st))

costs = [[-1 for _ in range(SIZE)] for _ in range(SIZE)]

while True:
    cost, pos = heapq.heappop(pq)
    costs[pos[0]][pos[1]] = cost

    dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)]

    if pos == end:
        print(cost)
        break

    if m[pos[0]][pos[1]] == "#" or m[pos[0]][pos[1]] == "O":
        continue

    m[pos[0]][pos[1]] = "O"

    for dir in dirs:
        np = (pos[0] + dir[0], pos[1] + dir[1])

        if (
            np[0] >= 0
            and np[0] < SIZE
            and np[1] >= 0
            and np[1] < SIZE
            and m[np[0]][np[1]] == "."
        ):
            heapq.heappush(pq, (cost + 1, np))

draw()
