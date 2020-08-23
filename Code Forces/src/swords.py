def gcd(x, y):
   while(y):
       x, y = y, x % y
   return x

input()
nums = [int(i) for i in str.split(input(), ' ')]

if len(nums) == 2:
    print(1, max(nums) - min(nums))
else:
    m = max(nums)
    rem = [m - n for n in nums]
    z = gcd(rem[0], rem[1])
    for i in range(2, len(rem)):
        z = gcd(z, rem[i])

    y = 0
    for n in nums:
        y += (m - n) / z
    y = int(y)
    print(y, z)


