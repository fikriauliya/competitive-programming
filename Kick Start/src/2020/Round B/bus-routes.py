t=int(input())
for tt in range(t):
    d = [int(i) for i in str.split(input(), ' ')][1]
    xs = [int(i) for i in str.split(input(), ' ')]
    res = [0] * len(xs)
    res[len(res)-1] = d//xs[len(xs)-1] * xs[len(xs)-1]
    for i in range(len(xs) - 2, -1, -1):
        res[i] = res[i+1]//xs[i] * xs[i]
    print("Case #{}: {}".format(tt + 1, res[0]))

