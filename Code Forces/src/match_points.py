_, z = map(int, input().split())
p = list(map(int, input().split()))
p.sort()
pos = -1
for i in range(len(p)//2, len(p)):
    if p[i] >= p[0] + z:
        pos = i
        break

if pos == -1:
    print(0)
else:
    # print(z, p, pos)
    r = pos
    l = 0
    ctr = 0
    while True:
        if l == pos:
            break
        if r == len(p):
            break
        if p[r] >= p[l] + z:
            ctr += 1
            r += 1
            l += 1
        else:
            r += 1
    print(ctr)
