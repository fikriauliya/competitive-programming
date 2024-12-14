from collections import defaultdict
from math import log10, floor

nums = list(map(int, input().split()))
mems = defaultdict(lambda: defaultdict(None))


def get(num, step):
    if step == 0:
        return 1

    if num in mems and step in mems[num]:
        return mems[num][step]

    digit = 1 if num == 0 else floor(log10(num)) + 1

    res = 0
    if num == 0:
        res = get(1, step - 1)

    elif digit % 2 == 0:
        a = num // (10 ** (digit / 2))
        b = num % (10 ** (digit / 2))
        res = get(a, step - 1) + get(b, step - 1)
    else:
        res = get(num * 2024, step - 1)

    mems[num][step] = res
    return res


res = 0
layer = 75
for num in nums:
    res += get(num, layer)

print(res)


# 0 1 2048
# 20 48
# 2 0 4 8
# 4048 1 9096 18192
# 40 48 1 90 96 (*2048)
# 4 0 4 8 2048 9 0 9 6
