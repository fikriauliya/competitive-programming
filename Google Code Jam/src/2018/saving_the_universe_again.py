def calc(p):
    damage = 0
    s = 1
    for i in range(len(p)):
        if p[i] == 'S':
            damage += s
        else:
            s *= 2
    return damage


def solve():
    d, p = input().split(' ')
    d = int(d)
    cost = 0
    damage = calc(p)
    while True:
        if damage <= d:
            return cost

        pos = str.find(p[::-1], 'SC')
        if pos == -1:
            return "IMPOSSIBLE"

        pos = len(p) - pos - 2
        p = p[:pos] + 'SC' + p[pos+2:]
        cost += 1

        damage = calc(p)


t = int(input())
for tt in range(1, t + 1):
    print("Case #{}: {}".format(tt, solve()))
