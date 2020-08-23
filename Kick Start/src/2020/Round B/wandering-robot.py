t=int(input())
for tt in range(t):
    w,h,l,u,r,d = [int(i) for i in str.split(input(), ' ')]
    # print(w,h,l,u,r,d)
    l-=1
    u-=1
    r-=1
    d-=1
    row = [1.0] * w
    for i in range(h-1, -1, -1):
        print(row)
        for j in range(w-1, -1, -1):
            if i >= u and i <= d and j >= l and j <= r:
                row[j] = 0
            elif j == w-1:
                continue
            elif i == h-1:
                row[j] = row[j+1]
            else:
                row[j] = (row[j] + row[j+1])/2

    print("Case #{}: {}".format(tt+1, float(row[0])))

