def rec(target, cur, seq):
    if len(seq) == 0:
        return cur == target

    return rec(target, cur * seq[0], seq[1:]) or rec(target, cur + seq[0], seq[1:])


total = 0
while True:
    try:
        line = input()
        s, seq = line.split(": ")
        s = int(s)
        nums = list(map(int, seq.split(" ")))

        if rec(s, nums[0], nums[1:]):
            total += s

    except EOFError:
        break

print(total)
