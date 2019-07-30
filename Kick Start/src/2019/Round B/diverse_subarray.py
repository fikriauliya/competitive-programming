t = int(input())
for tt in range(1, t+1):
    n, s = map(int, input().split(' '))
    a = list(map(int, input().split(' ')))
    # print(n, s, a)

    max_allowed = 0
    for i in range(n):
        freq = {}
        allowed = 0
        for j in range(i, n):
            if a[j] in freq:
                freq[a[j]] += 1
            else:
                freq[a[j]] = 1

            if freq[a[j]] <= s:
                allowed += 1
                max_allowed = max([max_allowed, allowed])
            elif freq[a[j]] == s + 1:
                allowed -= (freq[a[j]] - 1)
        #     print(j, a[j], freq[a[j]], allowed, end=" | ")
        # print()
        # print(i, freq, max_allowed)

        max_allowed = max([max_allowed, allowed])
    print("Case #{}: {}".format(tt, max_allowed))
