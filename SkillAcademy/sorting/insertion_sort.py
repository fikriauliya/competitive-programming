class InsertionSort:
    def __init__(self, items, animator):
        self._items = items
        self._animator = animator

    def sort(self):
        items = self._items
        #region UI
        animator = self._animator
        animator.pause()
        #endregion

        for i in range(1, len(items)):
            to_be_inserted = items[i]
            correct_position = i

            #region UI
            animator.set_label(
                f"Take out {to_be_inserted}. We need to find out correct position for {to_be_inserted}"
            )
            animator.draw(compared_indexes=[i], sorted_indexes=range(i))
            animator.pause()
            #endregion
            items[i] = None
            #region UI
            animator.pause()
            #endregion

            for j in range(i - 1, -1, -1):
                #region UI
                animator.set_label(f"Compare {items[j]} with {to_be_inserted}")
                animator.draw(compared_indexes=[j], sorted_indexes=range(i))
                animator.pause()
                #endregion

                if to_be_inserted < items[j]:
                    #region UI
                    animator.set_label(
                        f"{items[j]} is larger than {to_be_inserted}. Move {items[j]} to the right"
                    )
                    animator.pause()
                    #endregion
                    items[j + 1] = items[j]
                    items[j] = None
                    correct_position = j
                    #region UI
                    animator.pause()
                    #endregion
                else:
                    #region UI
                    animator.set_label(
                        f"{items[j]} is smaller than {to_be_inserted}. They are correctly ordered"
                    )
                    animator.pause()
                    #endregion
                    break

            #region UI
            animator.set_label(
                f"Insert {to_be_inserted} to position {correct_position}")
            animator.draw(sorted_indexes=range(i + 1))
            animator.pause()
            #endregion
            items[correct_position] = to_be_inserted
            #region UI
            animator.pause()
            #endregion
