import math

n =  int(input())

for i in range(n):
    k = int(input())
    ctr = 1
    l = 1
    m = 1
    while l < k:
        l += math.log(ctr, 10) + 1
        ctr += 1
        if ctr > m:
            ctr = 1
            m += 1
    print(ctr)
