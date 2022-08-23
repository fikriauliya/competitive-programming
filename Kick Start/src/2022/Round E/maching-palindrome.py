def is_palindrome(s):
    for i in range(len(s) // 2):
        if s[i] != s[-i - 1]:
            return False
    return True


t = int(input())
for tt in range(t):
    n = input()
    s = input()
    found = False
    for i in range(1, len(s)):
        if is_palindrome(s[:i]) and is_palindrome(s[i:]):
            print("Case #{}: {}".format(tt+1, s[:i]))
            found = True
            break
    if not found:
        print("Case #{}: {}".format(tt+1, s))
