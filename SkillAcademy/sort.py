from time import sleep
import curses


class Animator:
    def __init__(self, stdscr, items):
        self._stdscr = stdscr
        self._statusscr = curses.newwin(1, curses.COLS, 2, 0)
        self._items = items
        self._last_compared_indexes = []
        self._last_sorted_indexes = []
        self.COMPARED_ATTRIBUTES = curses.A_UNDERLINE | curses.color_pair(1)
        self.SORTED_ATTRIBUTES = curses.A_BOLD | curses.color_pair(2)

    def set_label(self, label):
        self._statusscr.clear()
        self._statusscr.addstr(0, 0, label)
        self._statusscr.refresh()

    def _draw_item(self, item, attributes=None):
        if item is None:
            item = ''

        if attributes:
            self._stdscr.addstr(f'{item}', attributes)
        else:
            self._stdscr.addstr(f'{item}')

    def draw(self, compared_indexes=[], sorted_indexes=[]):
        self._last_compared_indexes = compared_indexes
        self._last_sorted_indexes = sorted_indexes

        self._stdscr.move(0, 0)
        for (i, item) in enumerate(self._items):
            if i in compared_indexes:
                self._draw_item(item, self.COMPARED_ATTRIBUTES)
            elif i in sorted_indexes:
                self._draw_item(item, self.SORTED_ATTRIBUTES)
            else:
                self._draw_item(item)
            self._draw_item('\t')
        self._stdscr.refresh()

    def pause(self):
        self.draw(self._last_compared_indexes, self._last_sorted_indexes)
        self._stdscr.getch()


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


def main(stdscr):
    def init_ui():
        curses.init_pair(1, curses.COLOR_RED, curses.COLOR_BLACK)
        curses.init_pair(2, curses.COLOR_CYAN, curses.COLOR_BLACK)
        curses.curs_set(0)

    def simulate():
        sorter_classes = [InsertionSort]
        for sorter_class in sorter_classes:
            items = [5, 3, 6, 2, 7, 2, 9, 10]
            animator = Animator(stdscr, items)
            sorter = sorter_class(items, animator)
            sorter.sort()

    def wait_for_termination_key():
        while str.upper(chr(stdscr.getch())) != 'X':
            sleep(0.1)

    init_ui()
    simulate()
    wait_for_termination_key()


curses.wrapper(main)
