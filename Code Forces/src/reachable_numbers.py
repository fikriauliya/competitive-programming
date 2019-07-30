def f(n):
    n = n + 1
    while n % 10 == 0:
        n = n // 10
    return n


n = int(input())
ctr = 0
nums = {}
while n not in nums:
    print(n)
    nums[n] = True
    ctr += 1
    n = f(n)

print(ctr)
