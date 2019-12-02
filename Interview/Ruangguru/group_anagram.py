from collections import defaultdict

def group_anagram(words):
    mapping = defaultdict(list)
    for word in words:
        mapping[''.join(sorted(word))].append(word)
    return list(mapping.values())

print(group_anagram(['aku', 'kau', 'kua', 'muka', 'kamu', 'makan']))

