def generate_max_finder_code(n):
    if n < 2: return
    if n > 26: return
    syms = list(map(chr, range(ord('a'), ord('a') + n)))

    def rec(i,  max_candidate):
        if i > n: return
        cur_candidate = syms[i - 1]
        indentation = "   " * (i - 2)
        if n == i:
            print(indentation + f"if {max_candidate} > {cur_candidate}")
            print(indentation + "   " + f"max = {max_candidate}")
            print(indentation + f"else")
            print(indentation + "   " + f"max = {cur_candidate}")
        else:
            print(indentation + f"if {max_candidate} > {cur_candidate}")
            rec(i + 1, max_candidate)
            print(indentation + "else")
            rec(i + 1, cur_candidate)

    rec(2, 'a')

generate_max_finder_code(4)
