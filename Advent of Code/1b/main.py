from collections import Counter

alist = []
blist = []

while True:
    try:
        a, b = map(int, input().split())
        alist.append(a)
        blist.append(b)
    except EOFError:
        break

counter = Counter(blist)

total = 0

for a in alist:
    total += a * counter.get(a, 0)

print(total)
