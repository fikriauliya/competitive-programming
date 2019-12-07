from time import sleep
import curses


class SortAnimator:
    def __init__(self, stdscr):
        self._stdscr = stdscr
        self._statusscr = curses.newwin(1, curses.COLS, 2, 0)

    def on_swap(self, items, a, b):
        self._statusscr.addstr(0, 0, f"Swap {items[a]} {items[b]}")

    def on_loop_enter(self, items, compared_indexes, sorted_indexes):
        self._stdscr.move(0, 0)
        for (i, item) in enumerate(items):
            if i in compared_indexes:
                self._stdscr.addstr(f'{item}',
                                    curses.A_UNDERLINE | curses.color_pair(1))
            elif i in sorted_indexes:
                self._stdscr.addstr(f'{item}',
                                    curses.A_BOLD | curses.color_pair(2))
            else:
                self._stdscr.addstr(f'{item}')
            self._stdscr.addstr(' ')
        self._stdscr.refresh()
        self._statusscr.clear()

    def on_loop_end(self, items):
        self._stdscr.refresh()
        self._statusscr.refresh()
        self._stdscr.getch()

    def on_sort_end(self, items):
        self.on_loop_enter(len(items), -1)


class BubbleSort:
    def __init__(self, items):
        self._items = items
        self._subscriber = None

    def _swap(self, items, i, j):
        items[i], items[j] = items[j], items[i]

    def set_subscriber(self, subscriber):
        self._subscriber = subscriber

    def sort(self):
        items = self._items
        last_unsorted_index = len(items) - 1

        while True:
            swapped = False
            next_last_unsorted_index = None

            for i in range(last_unsorted_index):
                #region UI update
                self._on_loop_enter(i, last_unsorted_index)
                #endregion
                if items[i] > items[i + 1]:
                    self._swap(items, i, i + 1)
                    #region UI update
                    self._on_swap(i, i + 1)
                    #end region
                    next_last_unsorted_index = i
                    swapped = True
                #region UI update
                self._on_loop_end()
                #endregion
            if not swapped: break

            last_unsorted_index = next_last_unsorted_index

        #region UI Update
        self._on_sort_end()
        #endregion
        return items

    def _on_swap(self, i, j):
        if self._subscriber: self._subscriber.on_swap(self._items, i, j)

    def _on_loop_enter(self, i, last_unsorted_index):
        sorted_indexes = set(range(last_unsorted_index + 1, len(self._items)))
        compared_indexes = range(i, i + 2)
        if self._subscriber:
            self._subscriber.on_loop_enter(self._items, compared_indexes,
                                           sorted_indexes)

    def _on_loop_end(self):
        if self._subscriber: self._subscriber.on_loop_end(self._items)

    def _on_sort_end(self):
        if self._subscriber: self._subscriber.on_sort_end(self._items)


def main(stdscr):
    def init_ui():
        curses.init_pair(1, curses.COLOR_RED, curses.COLOR_BLACK)
        curses.init_pair(2, curses.COLOR_CYAN, curses.COLOR_BLACK)
        curses.curs_set(0)

    def simulate():
        items = [5, 3, 6, 2, 7, 2, 9, 10]
        animator = SortAnimator(stdscr)
        sorter = BubbleSort(items)
        sorter.set_subscriber(animator)
        sorter.sort()

    def dispatch_ui():
        while str.upper(chr(stdscr.getch())) != 'X':
            sleep(0.1)

    init_ui()
    simulate()
    dispatch_ui()


curses.wrapper(main)