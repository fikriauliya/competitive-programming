t=int(input())
for tt in range(t):
    input()
    h = [int(i) for i in str.split(input(), ' ')]
    res = 0
    for i in range(1, len(h)-1):
        if h[i] > h[i-1] and h[i] > h[i+1]:
            res += 1

    print("Case #{}: {}".format(tt + 1, res))
