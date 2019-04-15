mem = {}


class SeventhPowers:
    def reconstructA(self, B):
        if B in mem:
            return mem[B]
        if B < 0:
            return None
        if B == 0:
            return "0"
        else:
            for i in range(9, -1, -1):
                rem = B - i**7
                if rem == 0:
                    mem[B] = str(i)
                    return str(i)
                res = self.reconstructA(rem)
                mem[rem] = res
                if (res is not None):
                    return str(i) + res
            return None


print(SeventhPowers().reconstructA(0))
print(SeventhPowers().reconstructA(1))
print(SeventhPowers().reconstructA(2))
print(SeventhPowers().reconstructA(3))
print(SeventhPowers().reconstructA(7))
print(SeventhPowers().reconstructA(8))
print(SeventhPowers().reconstructA(49))
print(SeventhPowers().reconstructA(128))
print(SeventhPowers().reconstructA(130))  # 121
print(SeventhPowers().reconstructA(839927))
