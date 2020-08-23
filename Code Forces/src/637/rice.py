t=int(input())
for tt in range(t):
    n,a,b,c,d = [int(i) for i in str.split(input(), ' ')]
    # print(n,a,b,c,d)
    each = (a-b, a+b)
    total = (c-d,c+d)
    # print(each, total)
    # e = (total[0]//each[0]*each[0], total[1]//each[1]*each[1])
    tot = (each[0]*n, each[1]*n)
    if tot[0] >= total[0] and tot[0] <= total[1]:
        print("Yes")
    elif tot[1] >= total[0] and tot[1] <= total[1]:
        print("Yes")
    elif tot[0] <= total[0] and tot[1] >= total[1]:
        print("Yes")
    else:
        print("No")
