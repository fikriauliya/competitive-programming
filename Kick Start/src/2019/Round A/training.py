def solve():
    s.sort()
    min_sum = sum(list(map(lambda x: s[p-1] - x, s[:p-1])))
    prev_sum = min_sum
    for i in range(p, len(s)):
        cur_sum = prev_sum - (s[i-1] - s[i-p]) + (s[i]-s[i-1]) * (p-1)
        prev_sum = cur_sum
        min_sum = min([cur_sum, min_sum])
    return min_sum


t = int(input())  # read a line with a single integer
for i in range(1, t + 1):
    n, p = [int(s) for s in input().split(' ')]
    s = [int(s) for s in input().split(' ')]
    print("Case #{}: {}".format(i, solve()))
