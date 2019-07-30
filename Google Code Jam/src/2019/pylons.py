def solve():
    r, c = [int(inp) for inp in input().split(' ')]
    m = [[False] * c for x in range(0, r)]

    def rec(i):
        if i == r*c - 1:
            return
        else:
            for i in range(0, r):
                for j in range(0, c):

    return rec(0)


t = int(input())  # read a line with a single integer
for i in range(1, t + 1):
    print("Case #{}:".format(i, solve()))
