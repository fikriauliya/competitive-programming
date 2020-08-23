t=int(input())
for tt in range(t):
    n = int(input())
    p = [int(i) for i in str.split(input(), ' ')]
    indexes = {}
    for i in range(len(p)):
        indexes[p[i]] = i

    i = indexes[1]
    ctr = 1
    valid = True
    # print(p)

    while ctr <= n:
        if i == n or p[i] < ctr:
            i = indexes[ctr]
        # print(i)
        if p[i] == ctr:
            ctr += 1
            i += 1
        else:
            valid = False
            break


    if valid: print("Yes")
    else: print("No")

