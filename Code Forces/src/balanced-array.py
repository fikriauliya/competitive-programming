t=int(input())
for tt in range(t):
    n = int(input())
    if (n // 2) % 2 != 0:
        print("NO")
    else:
        a = []
        b = []
        for i in range(2, n+1, 2):
            a.append(i)
            if i <= n//2:
                b.append(i - 1)
            else:
                b.append(i + 1)

        a.extend(b)
        print("YES")
        print(' '.join(map(str, a)))



