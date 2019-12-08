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
