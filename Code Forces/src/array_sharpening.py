import heapq

tt = int(input())
for t in range(tt):
    input()
    arr = list(map(int, input().split(' ')))
    left = 0
    right = len(arr) - 1
    val_left = min(arr[0], 0)
    val_right = min(arr[-1], 0)
    while val_left < 0 and left < len(arr) - 1:
        if arr[left+1] > val_left:
            left += 1
            val_left = min(arr[left], 0)

    while val_right < 0 and right > 0:
        if arr[right-1] > val_right:
            right -= 1
            val_right = min(arr[right], 0)

    arr_i = list(zip(arr[left+1:right], range(left+1, right)))
    # left += 1
    # right -= 1
    print(arr)
    heapq.heapify(arr_i)
    if len(arr_i) == 0:
        print("Yes")
        continue
    matched = False
    while left < right - 1:
        print(arr_i)
        val, i = heapq.heappop(arr_i)
        print(" ", val_left, val, val_right, left, i, right)
        if val - val_left >= i - left:
            val_left += 1
            left = i
            matched = True
        if val - val_right >= right - i:
            val_right += 1
            right = i
            matched = True
        print(">", val_left, val, val_right, left, i, right)
        if not matched:
            break
    if matched:
        print("Yes")
    else:
        print("No")




