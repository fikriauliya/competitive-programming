n, k = [int(i) for i in str.split(input(), ' ')]
ds = [int(input(), 2) for i in range(n)]
nums = [0b1110111, 0b0010010, 0b1011101, 0b1011011, 0b0111010, 0b1101011, 0b1101111, 0b1010010, 0b1111111, 0b1111011]
index_nums = {}
for i, num in enumerate(nums):
    index_nums[num] = i


# print(index_nums)

# print([bin(num) for num in nums])
# print(n, k)

set_bits = {}
for i in range(2**7):
    count = 0
    j = i
    while j:
        j &= (j - 1)
        count += 1

    set_bits[i] = count

costs = [[0 for i in range(10)] for j in range(10)]
for i in range(10):
    for j in range(10):
        costs[i][j] = set_bits[nums[i]^nums[j]]

# print(costs)
# print(set_bits)

# print([set_bits[num] for num in nums])

diffs = []
for d in ds:
    res = costs[index_nums[d]]
    min_change = min(res)
    changed_to = nums[len(res) - 1 - res[::-1].index(min_change)]
    k -= min_change
    # print(k, bin(d), changed_to, res)

    new_d = changed_to
    diffs.append(index_nums[changed_to])

# print(ds)
# print(diffs)

def rec(k, i):
    if k == 0 and i == len(diffs):
        return True
    elif k < 0:
        return False
    else:
        found = False
        for j in range(9, -1, -1):
            prev = diffs[i]
            cost = costs[diffs[i]][j]
            # print(i, diffs[i], j, cost)
            diffs[i] = j
            if not rec(k - cost, i+1):
                diffs[i] = prev
            else:
                found = True
                break
        return found


if k < 0:
    print(-1)
else:
    rec(k, 0)
    print(''.join([str(i) for i in diffs]))


