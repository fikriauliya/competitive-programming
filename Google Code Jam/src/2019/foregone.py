def toZero(c):
    if (c != '4'):
        return '0'
    return c


def solve():
    n = input()
    a = int("".join(map(toZero, str(n))).replace('4', '2'))
    b = int(str(n).replace('4', '2'))
    print("Case #{}: {} {}".format(i, a, b))


t = int(input())  # read a line with a single integer
for i in range(1, t + 1):
    solve()
