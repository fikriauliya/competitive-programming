t=int(input())
for tt in range(t):
    b = [int(i) for i in str.split(input(), ' ')][1]
    a = [int(i) for i in str.split(input(), ' ')]
    a.sort()
    ctr = 0
    while ctr < len(a) and b >= a[ctr]:
        b -= a[ctr]
        ctr += 1
    print("Case #{}: {}".format(tt + 1, ctr))
