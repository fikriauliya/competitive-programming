from math import log10, floor

nums = list(map(int, input().split()))

for i in range(25):
    new_nums = []
    for num in nums:
        digit = 1 if num == 0 else floor(log10(num)) + 1

        if num == 0:
            new_nums.append(1)
        elif digit % 2 == 0:
            a = num // (10 ** (digit / 2))
            b = num % (10 ** (digit / 2))
            new_nums.append(a)
            new_nums.append(b)
        else:
            new_nums.append(num * 2024)
    nums = new_nums

print(len(nums))
