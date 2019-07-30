import itertools


def solve():
    input()
    n = {int(i) for i in str.split(input())}
    so = sorted(list(n))
    if len(n) == 1:
        return 0
    if len(n) == 2:
        if (so[1] + so[0]) % 2 == 0:
            return so[1] - (so[1] + so[0]) // 2
        else:
            return so[1] - so[0]
    if len(n) == 3 and (so[2] - so[1] == so[1] - so[0]):
        return so[1] - so[0]
    return -1


print(solve())
