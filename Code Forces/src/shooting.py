import itertools

input()
nums = [int(i) for i in str.split(input(), ' ')]
nums = list(zip(nums, itertools.count(1)))
nums = sorted(nums, key = lambda x:-x[0])

res = 0
for i in range(len(nums)):
    res += nums[i][0] * i + 1

print(res)
print(' '.join([str(r[1]) for r in nums]))
