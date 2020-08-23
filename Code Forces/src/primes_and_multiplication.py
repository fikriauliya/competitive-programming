import math
import functools

def primeFactors(n):
    res = set()
    while n % 2 == 0:
        res.add(2)
        n = n / 2
    for i in range(3,int(math.sqrt(n))+1,2):
        while n % i== 0:
            res.add(i)
            n = n / i
    if n > 2:
        res.add(int(n))
    return sorted(list(res))

def g(x, p):
    res = 0
    # print(x, p)
    while (x % p == 0):
        res += 1
        x = x / p
    return pow(p, res)

def f(x, y):
    m = ([g(y, xx) for xx in primeFactors(x)])
    return functools.reduce(lambda x, y: x * y, m)

x, n = [int(i) for i in str.split(input())]
xx = primeFactors(x)
print(x, n)
print(xx)
res = 1
for i in range(1, n+1):
    curRes = f(x, i)
    if curRes != 1:
        print('f(', x, ',', i, ')', curRes)
        res = res * curRes
        res = res % 1000000007
print(res)

