def solve():
    n = int(input())
    s = input()
    res = s.replace('S', 'X').replace('E', 'S').replace('X', 'E')
    print("Case #{}: {}".format(i, res))

t = int(input()) # read a line with a single integer
for i in range(1, t + 1):
    solve()
