t=int(input())
for tt in range(t):
    n,k = [int(i) for i in str.split(input(), ' ')]
    a = [int(i) for i in str.split(input(), ' ')]
    peaks = []
    # print(k)
    # print(a)
    peaks.append(0)
    for i in range(1, len(a)-1):
        if a[i] > a[i-1] and a[i] > a[i+1]:
            peaks.append(1)
        else:
            peaks.append(0)
    peaks.append(0)
    accums = []
    ctr = 0
    # print(peaks)

    for i in range(0, len(peaks)):
        accums.append(ctr + peaks[i])
        ctr += peaks[i]
    # print(accums)

    maxs = []
    max_val = 0
    max_pos = 0
    for i in range(0, len(accums) - k + 1):
        cur_max = accums[i+k-1] -accums[i]
        if peaks[i]: cur_max -= 1
        if peaks[i+k-1]: cur_max -= 1
        maxs.append(cur_max)
    # print(maxs)

    print(max(maxs)+1, maxs.index(max(maxs))+1)

    # print()
