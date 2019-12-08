class SelectionSort:
    def __init__(self, items, animator):
        self._items = items
        self._animator = animator

    def sort(self):
        items = self._items
        #region UI
        animator = self._animator

        #endregion

        def swap(items, x, y):
            items[x], items[y] = items[y], items[x]

        for i in range(len(items) - 1):
            minimum_index = i
            for j in range(i, len(items)):
                animator.draw(compared_indexes=[minimum_index, j],
                              sorted_indexes=range(i))
                animator.pause()
                if items[j] < items[minimum_index]:
                    animator.set_label(
                        f"{items[j]} is less than {items[minimum_index]}. Update the minimum value to {items[j]}"
                    )
                    animator.pause()
                    minimum_index = j
                animator.set_label("")
            animator.set_label(f"Swap {items[i]} with {items[minimum_index]}")
            animator.draw(compared_indexes=[i, minimum_index],
                          sorted_indexes=range(i))
            animator.pause()
            swap(items, i, minimum_index)

        return items
