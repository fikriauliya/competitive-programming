import sys
nums = [int(i) for i in str.split(input(), ' ')]
nums.sort()
b = (nums[0] - nums[1] + nums[2]) // 2
a = (nums[0] - b)
c = (nums[1] - a)
print(a, b, c)
