from collections import defaultdict


class CountingSort:
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

        def draw_animations(i):
            animator.draw(items, compared_indexes=[i])
            animator2.draw(counters, compared_indexes=[items[i]])
            animator.pause()

        counters = [0] * (max(items) + 1)
        for i, item in enumerate(items):
            draw_animations(i)

            items[i] = None
            counters[item] += 1

            draw_animations(i)

        ctr = 0
        for i in range(len(counters)):
            cur_counter = counters[i]
            for j in range(cur_counter):
                draw_animations(ctr)
                items[ctr] = i
                counters[i] -= 1
                draw_animations(ctr)
                ctr += 1