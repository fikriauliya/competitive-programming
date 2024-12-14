from functools import reduce

nums = list(map(int, list(input())))
data, free = nums[0::2], nums[1::2]
data = [[i, data[i]] for i in range(len(data))]
final = []

while len(data) > 0:
    final.append(data.pop(0))

    if len(data) == 0:
        break
    slot = free.pop(0)

    while len(data) > 0:
        num, count = data[-1]
        if count <= slot:
            # take all
            final.append(data.pop())

            if slot == count:
                break

            slot -= count
        else:
            # take partial, slotnya penuh
            final.append([num, slot])
            data[-1][1] -= slot
            break

ctr = 0
total = 0
for num, count in final:
    while count > 0:
        total += num * ctr
        ctr += 1
        count -= 1

print(total)
