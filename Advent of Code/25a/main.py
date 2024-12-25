accum = []
ms = []
while True:
    try:
        line = input()
    except EOFError:
        break

    if len(line) == 0:
        ms.append(accum)
        accum = []
    else:
        accum.append(line)


locks = []
keys = []
for m in ms:
    is_key = False
    if m[0][0] == ".":
        is_key = True

    nums = []
    transposed = list(map(list, zip(*m)))

    for i in range(len(transposed)):
        nums.append(len([t for t in transposed[i] if t == "#"]) - 1)
    if is_key:
        keys.append(nums)
    else:
        locks.append(nums)

fits = 0
for k in keys:
    for l in locks:
        sums = [k[i] + l[i] for i in range(len(k))]
        if all(map(lambda x: x <= 5, sums)):
            fits += 1

print(fits)
