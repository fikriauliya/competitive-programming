import math
from collections import defaultdict, deque
from typing import DefaultDict, List

def sort(items):
    r_digit = 0
    contain_digit = True
    while contain_digit:
        digits: DefaultDict[int, deque] = defaultdict(deque)
        contain_digit = False
        for item in items:
            cur_digit = item // 10**r_digit % 10
            if cur_digit > 0: contain_digit = True
            digits[cur_digit].append(item)
        
        if not contain_digit: break
        new_items = []
        for i in range(10):
            while len(digits[i]) > 0:
                new_items.append(digits[i].popleft())
        print(new_items)
        r_digit += 1

arr = [234,532,3112,232,1223]
sort(arr)

