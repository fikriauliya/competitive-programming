from collections import defaultdict

class T19AutoSuggestion:
    def __init__(self, dictionaries):
        self._digit_to_words = {}
        self._build_mapper()

    def _build_mapper(self):
        flatten = lambda l: [item for sublist in l for item in sublist]
        char_to_num = zip(["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"], range(2, 10))

        from itertools import cycle
        char_to_num_dict = dict(flatten([zip(l[0], cycle(str(l[1]))) for l in char_to_num]))
        # char_to_num_dict = {'a': 2, 'b': 2, 'c': 2, 'd': 3, ...}

        digit_to_words = defaultdict(list)
        for word in dictionaries:
            digit = ''.join([char_to_num_dict[c] for c in word])
            digit_to_words[digit].append(word)

        self._digit_to_words = digit_to_words
        # digit_to_words = {'96753': ['world'], '782644878': ['ruangguru'], '43556': ['hello', 'gello'], ... }

    def suggest(self, digit):
        return self._digit_to_words[digit]


dictionaries = ["hello", "world", "happy", "morning", "ruangguru", "gello"]
engine = T19AutoSuggestion(dictionaries)
print(engine.suggest('43556')) # ['hello', 'gello']
print(engine.suggest('96753')) # ['world']
print(engine.suggest('1111')) # []
print(engine.suggest('23')) # []
