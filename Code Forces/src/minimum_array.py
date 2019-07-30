from bisect import bisect_left
n = int(input())
a = list(map(int, input().split(' ')))
b = list(map(int, input().split(' ')))
b.sort()
m = {i: 0 for i in range(0, n + 1)}
pairs = {i: n - i for i in range(0, n + 1)}
for bb in b:
    m[bb] += 1
c = []
for aa in a:
    found = False
    if m[pairs[aa]] > 0:
        m[pairs[aa]] -= 1
        c.append((pairs[aa] + aa) % n)
    else:
        bisect_left(b, pairs[aa] + 1)
        for i in range(pairs[aa] + 1, n):
            if m[i] > 0:
                pairs[aa] = i
                m[i] -= 1
                c.append((i + aa) % n)
                found = True
                break
        if not found:
            for i in range(0, pairs[aa]):
                if m[i] > 0:
                    pairs[aa] = i
                    m[i] -= 1
                    c.append((i + aa) % n)
                    break
print(" ".join(map(str, c)))
