import itertools

t = int(input())
for tt in range(t):
    s = input()
    h = {}
    for c in s:
        if c in h:
            h[c] += 1
        else:
            h[c] = 1
    res = ''
    ss = sorted(list(set(s)))
    l = ss[0:len(ss)//2]
    r = ss[len(ss)//2:len(ss)]
    l_c = 0
    r_c = 0
    # print()
    # print(ss, l, r, h)
    if len(ss) == 2 and ord(ss[0]) + 1 == ord(ss[1]):
        print("No answer")
        continue
    if len(ss) == 3 and ord(l[0]) + 1 == ord(r[0]):
        if ord(r[0]) + 1 == ord(r[1]):
            print("No answer")
            continue
        else:
            print(r[0] * h[r[0]] + r[1] * h[r[1]] + l[0] * h[l[0]])
            continue

    while (True):
        ok = False
        if r_c < len(r):
            res += (h[r[r_c]] * r[r_c])
            r_c += 1
            ok = True
        if l_c < len(l):
            res += (h[l[l_c]] * l[l_c])
            l_c += 1
            ok = True
        if not ok:
            break
    print(res)
