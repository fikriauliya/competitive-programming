import functools


def solve():
    n, m, _ = [int(i) for i in str.split(input(), ' ')]
    front = [int(i) for i in str.split(input(), ' ')]
    left = [int(i) for i in str.split(input(), ' ')]
    top = [[int(i) for i in str.split(input(), ' ')] for _ in range(n)]
    res = [[0 for i in range(m)] for j in range(n)]

    for i in range(n):
        for j in range(m):
            if top[i][j] == 1:
                res[i][j] = min(left[i], front[j])

    for i in range(n):
        print(functools.reduce(lambda x, y: str(x) + " " + str(y), res[i]))


solve()
