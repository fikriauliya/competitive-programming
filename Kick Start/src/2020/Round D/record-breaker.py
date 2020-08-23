t = int(input())
for tt in range(t):
    input()
    vs = list(map(int, input().split(" ")))
    largest = -1
    ctr = 0
    for i in range(0, len(vs)):
        if vs[i] > largest and (i == len(vs) - 1 or vs[i] > vs[i + 1]):
            ctr += 1
        largest = max(largest, vs[i])
    print('Case #{}: {}'.format(tt+1, ctr))