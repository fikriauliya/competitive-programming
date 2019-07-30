t = int(input())
for tt in range(1, t + 1):
    n = int(input())
    s = []
    e = []
    l = []
    for nn in range(n):
        ss, ee, ll = map(int, input().split(' '))
        s.append(ss)
        e.append(ee)
        l.append(ll)

    time = 0
    rems = {i for i in range(n)}
    # print(s, e, l, rems)
    score = 0
    while rems:
        max_winner = rems.pop()
        rems.add(max_winner)
        max_e = 0
        # print('time', time)
        for i in rems:
            ei = max([e[i] - l[i] * time, 0])
            # print(i, ei, end=' | ')
            max_rem_e = 0
            for j in rems:
                if i != j:
                    ej = max([e[j] - l[j] * (time + s[i]), 0])
                    max_rem_e = max([max_rem_e, ej])
                    # print(ej, end=' ')
            if max_rem_e + ei > max_e:
                max_e = max_rem_e + ei
                max_winner = i
            # print()
        score += max([e[max_winner] - l[max_winner] * time, 0])
        # print(max_winner, max_e, score, rems)
        time += s[max_winner]
        rems.remove(max_winner)
    print("Case #{}: {}".format(tt, score))
