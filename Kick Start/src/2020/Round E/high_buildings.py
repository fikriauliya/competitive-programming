def solve():
    if c > a or c > b: return "IMPOSSIBLE"
    if n == 1: return "1"
    l = [1 for i in range(n)]
    for i in range(a):
        l[i] = 2
    for i in range(a-1, a-c-1, -1):
        l[i] = 3
    for i in range(n-1, n-(b-c)-1, -1):
        if l[i] != 1:
            return "IMPOSSIBLE"
        l[i] = 2
    if b == c and n - a > 0:
        if n == 2: return "IMPOSSIBLE"
        if a == 1: return "IMPOSSIBLE"
        l[-1], l[a-1] = l[a-1], l[-1]
    
    if n == 2: l = list(map(lambda x: x-1, l))
    return " ".join(map(str, l))

t = int(input())
for tt in range(t):
    n, a, b, c = list(map(int, input().split(' ')))
    l = solve()
    print('Case #{}: {}'.format(tt+1, l))