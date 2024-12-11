ctr = 0
while True:
    try:
        nums = list(map(int, input().split()))
        diffs = [b - a for (a, b) in zip(nums, nums[1:])]
        all_decreasing = all(d >= 0 for d in diffs)
        all_increasing = all(d <= 0 for d in diffs)
        print(diffs)

        if not all_decreasing and not all_increasing:
            continue

        abs_diffs = [abs(d) for d in diffs]
        if max(abs_diffs) <= 3 and min(abs_diffs) >= 1:
            ctr += 1

    except EOFError:
        break

print(ctr)
