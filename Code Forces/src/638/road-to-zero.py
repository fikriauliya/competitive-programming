t=int(input())
for tt in range(t):
    x, y = [int(i) for i in str.split(input(), ' ')]
    a, b = [int(i) for i in str.split(input(), ' ')]
    cost = 0

    cost += abs(x - y) * a
    if 2 * a < b:
        cost += 2 * a * min(x, y)
    else:
        cost += b * min(x, y)
    print(cost)



