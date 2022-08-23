import math


def f(n):
    res = 1
    turn = 0
    while n > 2:
        res += 1

        turn += 1
        if turn % 2 == 1:
            # Human
            n -= 3
        else:
            # Computer
            n -= 2

    return res


t = int(input())
for tt in range(t):
    n = int(input())
    res = math.ceil(f(n)/2)
    print('Case #{}: {}'.format(tt+1, res))
