import re


def solve_2x2_linear(a11, a12, a21, a22, b1, b2):
    determinant = a11 * a22 - a12 * a21

    if abs(determinant) < 1e-10:
        return None  # No unique solution (singular matrix)

    x1 = (b1 * a22 - b2 * a12) / determinant
    x2 = (b2 * a11 - b1 * a21) / determinant

    return x1, x2


total = 0

while True:
    line = input()
    nums = []

    nums_a = re.findall(r"(\d+)", line)
    nums_a = list(map(int, nums_a))

    line = input()
    nums_b = re.findall(r"(\d+)", line)
    nums_b = list(map(int, nums_b))

    nums = [nums_a] + [nums_b]
    # transpose nums
    nums = list(map(list, zip(*nums)))

    line = input()
    prizes = re.findall(r"(\d+)", line)
    prizes = list(map(int, prizes))
    prizes = list(map(lambda x: x + 10000000000000, prizes))

    try:
        x1, x2 = solve_2x2_linear(*nums[0], *nums[1], *prizes)

        if x1 % 1 == 0 and x2 % 1 == 0:
            total += int(3 * x1 + x2)

        print(f"{x1:.2f} {x2:.2f}")
        input()
    except EOFError:
        break

print(total)
