import itertools
import math


def solve():
    n, t = [int(i) for i in str.split(input(), ' ')]
    sd = []
    for _ in range(n):
        ss, dd = [int(i) for i in str.split(input(), ' ')]
        sd.append((ss, dd))

    nx = [sdd[0] + sdd[1] * math.ceil((t - sdd[0]) / sdd[1]) for sdd in sd]
    for i in range(n):
        if (sd[i][0] >= t):
            nx[i] = sd[i][0]

    print(nx.index(min(nx)) + 1)


solve()
