from time import sleep
import curses


class BubbleSort:
    def __init__(self, stdscr, items):
        self._items = items
        self._stdscr = stdscr
        self._statusscr = curses.newwin(1, curses.COLS, 2, 0)

    def _swap(self, items, i, j):
        items[i], items[j] = items[j], items[i]

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
                else:
                    self._on_not_swap()
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
        self._statusscr.addstr(0, 0, f"Swap {self._items[i]} {self._items[j]}")

    def _on_not_swap(self):
        self._statusscr.clear()

    def _on_loop_enter(self, i, last_unsorted_index):
        underline_indexes = {i, i + 1}
        bold_indexes = set(range(last_unsorted_index + 1, len(self._items)))
        self._stdscr.move(0, 0)
        for (i, item) in enumerate(self._items):
            if i in underline_indexes:
                self._stdscr.addstr(f'{item}',
                                    curses.A_UNDERLINE | curses.color_pair(1))
            elif i in bold_indexes:
                self._stdscr.addstr(f'{item}',
                                    curses.A_BOLD | curses.color_pair(2))
            else:
                self._stdscr.addstr(f'{item}')
            self._stdscr.addstr(' ')
        self._stdscr.refresh()

    def _on_loop_end(self):
        self._stdscr.refresh()
        self._statusscr.refresh()
        self._stdscr.getch()

    def _on_sort_end(self):
        self._on_loop_enter(len(self._items), -1)


def main(stdscr):
    curses.init_pair(1, curses.COLOR_RED, curses.COLOR_BLACK)
    curses.init_pair(2, curses.COLOR_CYAN, curses.COLOR_BLACK)
    curses.curs_set(0)

    stdscr.clear()
    items = [5, 3, 6, 2, 7, 2, 9, 10]
    # items = [1, 2, 3, 4, 1, 2, 3]
    BubbleSort(stdscr, items).sort()

    while str.upper(chr(stdscr.getch())) != 'X':
        sleep(0.1)


curses.wrapper(main)