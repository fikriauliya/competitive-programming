import math

t = int(input())
for tt in range(t):
    n, k = list(map(int, input().split(' ')))
    intervals = []
    for nn in range(n):
        s, e = list(map(int, input().split(' ')))
        intervals.append((s, e))
    intervals = sorted(intervals, key=lambda x: x[0])
    # print(intervals)

    res = 0
    s = 0
    e = 0
    for i in intervals:
        if e >= i[0]:
            if e >= i[1]:
                # print("continue")
                continue
            else:
                s = e
        else:
            s = i[0]

        rem = i[1]-s
        times = math.ceil(float(rem) / k)
        e = s + times * k
        s = e - k
        # print(i, s, e, times)
        res += times

    print('Case #{}: {}'.format(tt+1, res))
