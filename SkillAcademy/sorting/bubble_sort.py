class BubbleSort:
    def __init__(self, items, animator):
        self._items = items
        self._animator = animator

    def sort(self):
        items = self._items
        #region UI
        animator = self._animator
        animator.pause()

        #endregion

        def swap(items, x, y):
            items[x], items[y] = items[y], items[x]

        last_unsorted_index = len(items) - 1

        while True:
            swapped = False
            next_last_unsorted_index = None

            for i in range(last_unsorted_index):
                #region UI update
                animator.draw(compared_indexes=[i, i + 1],
                              sorted_indexes=range(last_unsorted_index + 1,
                                                   len(items)))
                #endregion
                if items[i] > items[i + 1]:
                    #region UI update
                    animator.set_label(f"Swap {items[i]} with {items[i+1]}")
                    animator.pause()
                    #end region
                    swap(items, i, i + 1)
                    next_last_unsorted_index = i
                    swapped = True
                    #region UI update
                    animator.draw(compared_indexes=[i, i + 1],
                                  sorted_indexes=range(last_unsorted_index + 1,
                                                       len(items)))
                    #end region
                else:
                    animator.set_label(
                        f"{items[i]} and {items[i+1]} are correctly ordered. Do nothing"
                    )
                #region UI update
                animator.pause()
                #endregion
            if not swapped: break
            last_unsorted_index = next_last_unsorted_index

        animator.draw(sorted_indexes=range(0, len(items)))
        animator.pause()
        return items