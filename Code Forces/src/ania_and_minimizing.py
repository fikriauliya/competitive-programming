def f():
    n, k = [int(i) for i in str.split(input())]
    s = input()
    if k == 0:
        return s

    if len(s) == 1:
        return 0

    res = "1"
    if s[0] != '1':
        k -= 1

    for i in range(1, len(s)):
        if k > 0 and s[i] != '0':
            res += "0"
            k -= 1
        else:
            res += s[i]
    return res

print(f())

