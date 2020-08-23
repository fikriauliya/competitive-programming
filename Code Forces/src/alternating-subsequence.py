t=int(input())
for tt in range(t):
    input()
    a = [int(i) for i in str.split(input(), ' ')]
    cur_min = None
    cur_max = None
    res = 0
    for i in a:
        if i < 0:
            if cur_max is not None:
                # print(cur_max)
                res += cur_max
                cur_max = None
            if cur_min is not None:
                cur_min = max(cur_min, i)
            else:
                cur_min = i
        else:
            if cur_min is not None:
                # print(cur_min)
                res += cur_min
                cur_min = None
            if cur_max is not None:
                cur_max = max(cur_max, i)
            else:
                cur_max = i
    if cur_max is not None: res += cur_max
    if cur_min is not None: res += cur_min
    print(res)

