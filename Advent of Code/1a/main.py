alist = []
blist = []

while True:
    try:
        a, b = map(int, input().split())

        alist.append(a)
        blist.append(b)
    except EOFError:
        break

alist.sort()
blist.sort()

diff = [abs(a - b) for (a, b) in zip(alist, blist)]
print(sum(diff))
