a, b, c = [int(i) for i in str.split(input())]
l = [a, a, b, c, a, c, b]
w = [a//3, b//2, c//2]
m = min(w)
# print(m)
l[0] -= (m * 3)
l[1] -= (m * 3)
l[2] -= (m * 2)
l[3] -= (m * 2)
l[4] -= (m * 3)
l[5] -= (m * 2)
l[6] -= (m * 2)
# print(l)
m_max = -1
for i in range(len(l)):
    ctr = 0
    ll = l[:]
    for j in range(len(l)):
        index = (i + j) % len(l)
        if ll[index] <= 0:
            break
        else:
            if index in [0, 1, 4]:
                ll[0] -= 1
                ll[1] -= 1
                ll[4] -= 1
            elif index in [2, 6]:
                ll[2] -= 1
                ll[6] -= 1
            else:
                ll[3] -= 1
                ll[5] -= 1
            ctr += 1
        # print(ctr, index, ll)
    m_max = max([ctr, m_max])
print(m * 7 + m_max)
