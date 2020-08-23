t=int(input())
for tt in range(t):
    k = [int(i) for i in str.split(input(), ' ')][1]
    a = [int(i) for i in str.split(input(), ' ')]
    # print(k, a)

    intervals = []
    begins = []
    ends = []
    sums = {}
    for i in range(0, len(a)//2):
        x = a[i]
        y = a[len(a)-i-1]
        pair = min(x+1, y+1), max(x+k, y+k)
        s = x + y
        if s in sums:
            sums[s] += 1
        else:
            sums[s] = 1
        # print(pair)
        begins.append(pair[0])
        ends.append(pair[1])
        intervals.append(pair)
    begins.sort()
    ends.sort()
    # print(begins,ends)
    i = 0
    j = 0
    ctr = 0
    max_ctr = 0
    max_begin = 0
    max_end = 0
    while i<len(begins) and j<len(ends):
        if begins[i] <= ends[j]:
            ctr += 1
            if ctr > max_ctr:
                max_begin = begins[i]
                max_ctr = ctr
            i += 1
        else:
            if max_ctr == ctr:
                max_end = ends[j]
            ctr -= 1
            j += 1
    if max_end == 0:
        max_end = ends[0]

    res=len(a)

    max_sum = 0
    for i in range(max_begin, max_end+1):
        if i in sums:
            max_sum = max(sums[i], max_sum)
    # print(max_ctr, max_begin, max_end, sums, max_sum)
    # print(res)
    res -= max_sum * 2
    # print(res)
    res -= (max_ctr - max_sum)
    print(res)



