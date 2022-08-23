from bisect import bisect_right


def find_le(a, x):
    i = bisect_right(a, x)
    if i:
        return (a[i-1], i-1)
    raise ValueError


t = int(input())
for tt in range(t):
    n = int(input())
    r = list(map(int, input().split(' ')))
    sr = sorted(r)

    ress = []
    for item in r:
        (val, index) = find_le(sr, item * 2)
        if val == item:
            if index == 0:
                res = -1
            else:
                res = sr[index-1]
        else:
            res = val
        ress.append(res)

    print('Case #{}: {}'.format(tt+1, ' '.join(map(str, ress))))
