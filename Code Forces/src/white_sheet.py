def f():
    w_x1, w_y1, w_x2, w_y2 = [int(i) for i in str.split(input(), ' ')]
    for i in range(2):
        b_x1, b_y1, b_x2, b_y2 = [int(i) for i in str.split(input(), ' ')]

        if b_y1 <= w_y1 and b_y2 >= w_y2:
            if b_x1 <= w_x1 and b_x2 >= w_x1:
                if b_x2 >= w_x2:
                    print("NO")
                    return
                else:
                    w_x1 = b_x2
            elif b_x1 >= w_x1 and b_x1 <= w_x2:
                if b_x2 >= w_x2:
                    w_x2 = b_x1
        if b_x1 <= w_x1 and b_x2 >= w_x2:
            if b_y1 <= w_y1 and b_y2 >= w_y1:
                if b_y2 >= w_y2:
                    print("NO")
                    return
                else:
                    w_y1 = b_y2
            elif b_y1 >= w_y1 and b_y1 <= w_y2:
                if b_y2 >= w_y2:
                    w_y2 = b_y1
    print("YES")

f()
