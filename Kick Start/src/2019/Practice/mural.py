from itertools import accumulate
import operator
import math

t = int(input())
for tt in range(1, t+1):
    input()
    scores = [int(c) for c in input()]
    p_scores = [0] + list(accumulate(scores, operator.add))
    max_span = math.ceil(len(scores)/2)
    m = max([p_scores[i+max_span] - p_scores[i]
             for i in range(0, len(scores)-max_span+1)])
    print("Case #{}: {}".format(tt, m))
