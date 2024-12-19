import heapq

m = []
pos = (0, 0)
end = (0, 0)

while True:
    try:
        line = input()
        m.append(list(line))
    except EOFError:
        break

for i in range(len(m)):
    for j in range(len(m[i])):
        if m[i][j] == "S":
            pos = (i, j)
        if m[i][j] == "E":
            end = (i, j)

cost = 0

# priority queue
q = []

# cost, pos, direction
heapq.heappush(q, (0, pos, (0, 1)))
visited = {}


def add(a, b):
    return (a[0] + b[0], a[1] + b[1])


def in_bound(p):
    return p[0] >= 0 and p[0] < len(m) and p[1] >= 0 and p[1] < len(m[0])


def can_go(p, dir):
    return (
        in_bound(p) and (p not in visited or visited[p] != dir) and m[p[0]][p[1]] != "#"
    )


min_cost = -1
while q:
    c, p, dir = heapq.heappop(q)

    if p == end:
        min_cost = c
        print(c)
        break

    visited[p] = dir

    np = add(p, dir)
    if can_go(np, dir):
        heapq.heappush(q, (c + 1, np, dir))

    rot_dir = (dir[1], -dir[0])
    np = add(p, rot_dir)
    if can_go(np, rot_dir):
        heapq.heappush(q, (c + 1001, np, rot_dir))

    rot_dir = (-dir[1], dir[0])
    np = add(p, rot_dir)
    if can_go(np, rot_dir):
        heapq.heappush(q, (c + 1001, np, rot_dir))

paths = set()
best_paths = set()


def can_go2(p):
    return in_bound(p) and (p not in paths) and m[p[0]][p[1]] != "#"


def dfs(p, cost, dir):
    print(p, cost, dir)
    if p == end and cost == min_cost:
        for k in paths:
            best_paths.add(k)
        return

    if cost > min_cost:
        return

    paths.add(p)

    np = add(p, dir)
    if can_go2(np):
        dfs(np, cost + 1, dir)

    rot_dir = (dir[1], -dir[0])
    np = add(p, rot_dir)
    if can_go2(np):
        dfs(np, cost + 1001, rot_dir)

    rot_dir = (-dir[1], dir[0])
    np = add(p, rot_dir)
    if can_go2(np):
        dfs(np, cost + 1001, rot_dir)

    paths.remove(p)


dfs(pos, 0, (0, 1))
print(len(best_paths))
