def forever():
    sum_e = sum(es)
    for i in range(len(es)):
        if rs[i] > sum_e - es[i]:
            return (False, i)
    return (True, None)

def simulate():
    res = 0
    i = 0
    forgets = [0 for i in range(len(es))]
    while True:
        if forgets[i] > res:
            return (res, i)
        res += es[i]
        forgets[i] = res + rs[i]
        i = (i + 1) % len(es)

def solve(ctr):
    if len(rs) == 0: return (ctr, 0)
    res1, rem = simulate()

    del rs[rem]
    del es[rem]
    ctr2, res2 = solve(ctr + 1)
    if res2 > res1:
        return (ctr2, res2)
    else:
        return (ctr, res1)


t = int(input())
for tt in range(t):
    n = int(input())
    es, rs = [], []
    for nn in range(n):
        e, r = list(map(int, input().split(' ')))
        es.append(e)
        rs.append(r)

    ors, oes = rs.copy(), es.copy()

    fore = False
    ctr = 0
    while not fore and len(es) > 0:
        fore, rem = forever()
        if fore: break
        del rs[rem]
        del es[rem]
        ctr += 1
    if fore: 
        res = "INDEFINITELY"
    else:
        es, rs = oes, ors
        ctr, res = solve(0)

    print('Case #{}: {} {}'.format(tt+1, ctr, res))