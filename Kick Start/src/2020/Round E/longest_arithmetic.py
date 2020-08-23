t = int(input())
for tt in range(t):
    input()
    arr = list(map(int, input().split(" ")))
    ctr = 2
    max_ctr = ctr
    diff = arr[1] - arr[0]
    for i in range(2, len(arr)):
        if arr[i] - arr[i-1] == diff:
            ctr += 1
        else :
            ctr = 2
            diff = arr[i] - arr[i-1]
        max_ctr = max(ctr, max_ctr)

    print('Case #{}: {}'.format(tt+1, max_ctr))