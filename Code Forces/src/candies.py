t=int(input())
series = [2**k-1 for k in range(2, 50) if 2**k-1 <= 10**9]
for tt in range(t):
    n = int(input())
    for s in series:
        if n % s == 0:
            print(n//s)
            break
