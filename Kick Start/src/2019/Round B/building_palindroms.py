import itertools
import string

t = int(input())
for tt in range(1, t+1):
    n, q = map(int, input().split(' '))
    p = input()
    yes = 0

    char_count = {c: [0] for c in string.ascii_uppercase}
    for c in p:
        for alp in string.ascii_uppercase:
            char_count[alp].append(char_count[alp][-1])
            if c == alp:
                char_count[alp][-1] += 1

    for qq in range(q):
        l, r = map(int, input().split(' '))
        odd = 0
        for k, v in char_count.items():
            # print(l, r, p[l-1:r], k, v[r], v[l-1])
            if (v[r] - v[l-1]) % 2 == 1:
                odd += 1

        if (r - l + 1) % 2 == 0 and odd == 0:
            yes += 1
        if (r - l + 1) % 2 == 1 and odd == 1:
            yes += 1
    print("Case #{}: {}".format(tt, yes))
