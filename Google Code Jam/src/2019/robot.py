import itertools


def gcd(a, b):
    if a == 0:
        return b

    return gcd(b % a, a)


def lcm(a, b):
    return (a*b) / gcd(a, b)


def lcm_arr(arr):
    res = 1
    for a in arr:
        res = lcm(a, res)
    return res


def is_win(a, b):
    longest = int(lcm(len(a), len(b)))
    for i in range(longest):
        if a[i % len(a)] == 'S':
            if b[i % len(b)] == 'R':
                return False
            if b[i % len(b)] == 'P':
                return True
        if a[i % len(a)] == 'P':
            if b[i % len(b)] == 'S':
                return False
            if b[i % len(b)] == 'R':
                return True
        if a[i % len(a)] == 'R':
            if b[i % len(b)] == 'P':
                return False
            if b[i % len(b)] == 'S':
                return True
    return False


def solve():
    a = int(input())
    moves = []
    win = {'R': {'P', 'R'}, 'P': {'P', 'S'}, 'S': {'R', 'S'}}
    for _ in range(a):
        moves.append(input())
    longest = int(lcm_arr(map(len, moves)))

    res = ''
    for i in range(longest):
        win_c = None
        if len(moves) == 0:
            return res
        for move in moves:
            if win_c is None:
                win_c = win[move[i % len(move)]]
            else:
                win_c = win[move[i % len(move)]].intersection(win_c)
            if len(win_c) == 0:
                return 'IMPOSSIBLE'
        if len(win_c) == 1:
            res += list(win_c)[0]
        elif len(win_c) == 2:
            if win_c == {'P', 'R'}:
                res += 'P'
            elif win_c == {'P', 'S'}:
                res += 'S'
            elif win_c == {'R', 'S'}:
                res += 'R'
            return res

        new_moves = []
        for move in moves:
            if not is_win(res, move):
                new_moves.append(move)
        moves = new_moves

    for move in moves:
        if not is_win(res, move):
            return 'IMPOSSIBLE'
    return res


t = int(input())
for tt in range(1, t+1):
    print("Case #{}: {}".format(tt, solve()))
