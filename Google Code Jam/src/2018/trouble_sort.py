import itertools


def solve():
    input()
    a = list(map(int, str(input()).split(' ')))
    a_sorted = sorted(a)
    a_even = []
    a_odd = []
    for i in range(len(a)):
        if i % 2 == 0:
            a_even.append(a[i])
        else:
            a_odd.append(a[i])
    a_even.sort()
    a_odd.sort()
    a_new = []
    for i in range(len(a)):
        if i % 2 == 0:
            a_new.append(a_even[i//2])
        else:
            a_new.append(a_odd[i//2])
    for i in range(len(a)):
        if a_new[i] != a_sorted[i]:
            return i

    # print(a_new)
    # print(a_sorted)
    return "OK"


t = int(input())
for tt in range(t):
    print("Case #{}: {}".format(tt + 1, solve()))
