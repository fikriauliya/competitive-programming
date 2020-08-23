import operator

t=int(input())
m = {'S': (0, 1), 'N': (0, -1), 'W': (-1, 0), 'E': (1,0)}
for tt in range(t):
    pos = (1, 1)
    s = input()
    mults = []
    cur_mult = 1
    for c in s:
        if ord(c) in range(ord('2'), ord('9')+1):
            mults.append(int(c))
            cur_mult *= mults[-1]
        elif c == ')':
            cur_mult //= mults.pop()
        elif c == '(':
            continue
        else:
            move = tuple(map(lambda x: cur_mult * x, m[c]))
            pos = tuple(map(operator.add, pos, move))

    def r(n):
        if n in range(1, 10**9+1): return n
        return (n -1) % (10**9) + 1

    pos = tuple(map(r, pos))
    print("Case #{}: {} {}".format(tt + 1, *list(pos)))

# print(r(10**9))
# print(r(1))
# print(r(0))
# print(r(10**9+1))
# print(r(2*10**9))
# print(r(2*10**9+1))
