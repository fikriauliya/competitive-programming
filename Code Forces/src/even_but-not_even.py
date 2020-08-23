tt = int(input())
for t in range(tt):
    input()
    n = int(input())
    sn = str(n)
    s = sum([int(nn) for nn in sn])
    # print(n, s)
    if s % 2 != 0:
        for i, d in enumerate(sn[:-1]):
            if int(d) % 2 == 1:
                sn = sn[:i] + sn[i+1:]
                break

    n = int(sn)
    if n % 2 == 0:
        for i in range(len(sn) - 1, 0, -1):
            if int(sn[i]) % 2 == 1:
                sn = sn[:i+1]
                break

    s = sum([int(nn) for nn in sn])
    n = int(sn)
    if s % 2 == 0 and n % 2 == 1:
        print(n)
    else:
        print(-1)
