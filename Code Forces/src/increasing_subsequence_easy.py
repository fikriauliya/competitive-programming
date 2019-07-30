input()
a = list(map(int, input().split(' ')))
i = 0
j = len(a) - 1
seq = ''
min_num = 0
while i <= j:
    if min_num < max(a[i], a[j]):
        if a[i] < a[j]:
            if a[i] > min_num:
                seq += 'L'
                min_num = a[i]
                i += 1
            else:
                seq += 'R'
                min_num = a[j]
                j -= 1
        else:
            if a[j] > min_num:
                seq += 'R'
                min_num = a[j]
                j -= 1
            else:
                seq += 'L'
                min_num = a[i]
                i += 1
    else:
        break
print(len(seq))
print(seq)
