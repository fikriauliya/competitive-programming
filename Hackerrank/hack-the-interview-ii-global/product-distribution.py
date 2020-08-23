n, m = list(map(int, input().strip().split(' ')))
a = list(map(int, input().strip().split(' ')))
a.sort()

res = 0
i = 0
ctr = 1
while i < n:
    next_i = i + m
    if next_i + m >= n:
        next_i = n
    res += ctr * sum(a[i:next_i])
    i = next_i
    ctr += 1

print(res)
