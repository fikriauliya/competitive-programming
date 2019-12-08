from time import sleep


class MergeSort:
    def __init__(self, items, animator, animator2):
        self._items = items
        self._animator = animator
        self._animator2 = animator2

    def sort(self):
        items = self._items
        #region UI
        animator = self._animator
        animator2 = self._animator2

        #endregion

        def sort_rec(left, right):
            if left < right:
                mid = (left + right) // 2
                animator.set_label(
                    f"left = {left}, mid = {mid}, right = {right}")
                animator.draw(self._items,
                              left_indexes=range(left, mid + 1),
                              right_indexes=range(mid + 1, right + 1))
                animator.pause()
                sort_rec(left, mid)
                sort_rec(mid + 1, right)
                merge(left, mid, right)

        def merge(left, mid, right):
            left_index = left
            right_index = mid + 1
            res = []

            def draw_animators(compared_indexes=[]):
                animator.draw(self._items,
                              left_indexes=range(left, mid + 1),
                              right_indexes=range(mid + 1, right + 1),
                              compared_indexes=compared_indexes)
                animator2.draw(res)
                animator2.pause()

            animator2.set_label(f"Merging {left}..{mid} with {mid+1}..{right}")
            while left_index <= mid and right_index <= right:
                draw_animators([left_index, right_index])
                if items[left_index] < items[right_index]:
                    res.append(items[left_index])
                    items[left_index] = None
                    left_index += 1
                else:
                    res.append(items[right_index])
                    items[right_index] = None
                    right_index += 1

            remaining_index, remaining_bound = (
                left_index, mid) if left_index <= mid else (right_index, right)
            while remaining_index <= remaining_bound:
                draw_animators([remaining_index])
                res.append(items[remaining_index])
                items[remaining_index] = None
                remaining_index += 1

            draw_animators()
            self._items[left:right + 1] = res
            res = []
            animator2.clear()
            draw_animators()

        sort_rec(0, len(items) - 1)
