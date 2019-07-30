import itertools

t = int(input())
for tt in range(t):
    n, k, x1, y1, c, d, e1, e2, f = map(int, input().split(' '))
    x = []
    y = []
    a = []
    x.append(x1)
    y.append(y1)
    for i in range(1, n):
        x.append((c * x[i-1] + d * y[i-1] + e1) % f)
        y.append((d * x[i-1] + c * y[i-1] + e2) % f)
    a = list(itertools.starmap(lambda x, y: (x + y) % f, zip(x, y)))

    total = 0
    for kk in range(1, k + 1):
        for i in range(n):
            for j in range(i, n):
                for k in range(i, j + 1):
                    total += (a[k] * (k - i + 1) ** kk)
                    total = total % int(1e9 + 7)
    print("Case #{}: {}".format(tt + 1, total))
