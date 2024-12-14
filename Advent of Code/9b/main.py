from itertools import zip_longest
from functools import reduce

nums = list(map(int, list(input())))
raw_data, raw_free = nums[0::2], nums[1::2]

data: list[tuple[int | None, int]] = [(i, raw_data[i]) for i in range(len(raw_data))]
free: list[tuple[int | None, int]] = [(None, raw_free[i]) for i in range(len(raw_free))]

fill: tuple[int | None, int] = (None, 0)
segment: list[list[tuple[int | None, int]]] = [
    [a] + [b] for a, b in zip_longest(data, free, fillvalue=fill)
]
d: list[tuple[int | None, int]] = reduce(lambda x, y: x + y, segment)


def print_d():
    for i in range(len(d)):
        val = d[i][0]
        val = "." if val is None else str(val)
        print(val * d[i][1], end="")
    print()


for i in range(len(d) - 1, 0, -1):
    right = d[i]
    if right[0] is None:
        continue

    for j in range(i):
        left = d[j]

        if left[0] is not None:
            continue

        if int(left[1]) < int(right[1]):
            continue

        if right[1] == left[1]:
            # The same length, just move right to left
            d[j] = right
            d[i] = (None, right[1])

            break
        elif right[1] < left[1]:
            # right can be moved to left, but left some space
            d[j] = right
            d[i] = (None, right[1])

            d.insert(j + 1, (None, left[1] - right[1]))
            break

print_d()

ctr = 0
total = 0
for num, count in d:
    if num is None:
        ctr += count
    else:
        while count > 0:
            total += num * ctr
            ctr += 1
            count -= 1
print(total)
