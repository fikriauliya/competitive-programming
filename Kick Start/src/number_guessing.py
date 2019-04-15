import sys


def guess(a, b):
    if (a == b):
        print(a)
    else:
        mid = (a + b) // 2
        print(mid)
    sys.stdout.flush()

    verdict = input()
    if verdict in ['CORRECT', 'WRONG_ANSWER']:
        return
    elif verdict == 'TOO_SMALL':
        guess(mid + 1, b)
    elif verdict == 'TOO_BIG':
        guess(a, mid - 1)


if __name__ == "__main__":
    t = int(input())
    for tt in range(1, t+1):
        a, b = [int(i) for i in str.split(input(), ' ')]
        n = int(input())
        guess(a+1, b)
