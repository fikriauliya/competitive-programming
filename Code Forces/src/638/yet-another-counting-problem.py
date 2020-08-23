def gcd(a, b):
    if (a == 0): return b
    return gcd(b % a, a)

def lcm(a, b): return int((a * b) / gcd(a, b))

t=int(input())
for tt in range(t):
    a, b, q = [int(i) for i in str.split(input(), ' ')]
    # print(a, b)

    ctr = [0]
    lcm_res = lcm(a, b)
    for i in range(1, lcm_res):
        # print(i, (i % a) % b, (i % b) % a, end="\t")
        if (i % a) % b != (i % b) % a:
            ctr.append(ctr[-1]+1)
        else:
            ctr.append(ctr[-1])
    # print()
    # print(ctr)
    res = []
    for qq in range(q):
        l, r = [int(i) for i in str.split(input(), ' ')]
        cur_res = 0
        prev_l = l // lcm_res * lcm_res
        next_l = (1 + l // lcm_res) * lcm_res
        prev_r = r // lcm_res * lcm_res
        # print(prev_l, prev_r)

        if prev_l == prev_r:
            # print('a')
            if l - prev_l - 1 < 0:
                cur_res += ctr[r - prev_r]
            else:
                cur_res += ctr[r - prev_r] - ctr[l - prev_l - 1]
        else:
            # print('b',(ctr[-1] - ctr[l - prev_l - 1]), ctr[r - prev_r])
            if l - prev_l - 1 < 0:
                cur_res += ctr[-1] + ctr[r - prev_r]
            else:
                cur_res += (ctr[-1] - ctr[l - prev_l - 1]) + ctr[r - prev_r]

        if next_l < prev_r:
            # print('c', next_l, prev_r)
            cur_res += (prev_r - next_l) // lcm_res * ctr[-1]
        res.append(int(cur_res))
    print(' '.join(map(str, res)))
    # print()
