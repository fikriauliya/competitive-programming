class MissingDwarf:
    def getHeight(self, otherHeights):
        s = sum(otherHeights)
        for i in range(max(otherHeights) + 1, 1001):
            if (s + i) % 7 == 0:
                return i


print(MissingDwarf().getHeight([1, 2, 3, 4, 5, 6]))
print(MissingDwarf().getHeight([10, 10, 20, 20, 30, 30]))
