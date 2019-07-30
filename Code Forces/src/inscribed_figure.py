def solve():
    f.reverse()
    h = {
        (3, 1): 4,
        (3, 2): -1,
        (3, 3): -1,
        (2, 1): 3,
        (2, 2): -1,
        (2, 3): -1,
        (1, 1): -1,
        (1, 2): 3,
        (1, 3): 4
    }
    ctr = 0
    for i in range(len(f)-1):
        cur = h[(f[i], f[i+1])]
        if cur == -1:
            print("Infinite")
            return
        ctr += cur
    print("Finite")
    print(ctr)


input()
f = list(map(int, input().split()))
solve()
