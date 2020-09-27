import math
from collections import defaultdict
t = int(input())
for tt in range(t):
    n, x = list(map(int, input().split(' ')))
    a = list(map(int, input().split(' ')))
    a = [math.ceil(y / x) for y in a]
    u = sorted(list(set(a)))
    h = {}
    for i in range(len(a)):
        if a[i] in h:
            h[a[i]].append(i)
        else:
            h[a[i]] = [i]

    res = []
    for i in u:
        for j in h[i]:
            res.append(str(j+1))

    print("Case #" + str(tt+1) + ": " + " ".join(res))
