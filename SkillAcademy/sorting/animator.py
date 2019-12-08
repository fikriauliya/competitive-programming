import curses


class Animator:
    WIDTH_PER_ITEM = 5
    BAR_HEIGHT = 20

    def __init__(self, stdscr, items):
        self._stdscr = stdscr
        self._statusscr = curses.newwin(1, curses.COLS, 0, 0)
        self._numscr = curses.newwin(1, curses.COLS, 2, 0)
        self._barscr = curses.newwin(self.BAR_HEIGHT, curses.COLS, 3, 0)
        self._items = items
        self._maximum = max(items)

        self._last_compared_indexes = []
        self._last_sorted_indexes = []
        self.COMPARED_ATTRIBUTES = curses.A_UNDERLINE | curses.color_pair(1)
        self.SORTED_ATTRIBUTES = curses.A_BOLD | curses.color_pair(2)
        self.BAR_ATTRIBUTES = curses.A_REVERSE

    def set_label(self, label):
        self._statusscr.clear()
        self._statusscr.addstr(0, 0, label)
        self._statusscr.refresh()

    def _draw_item(self, index, item, attributes=None):
        if item is None: return

        self._numscr.move(0, index * self.WIDTH_PER_ITEM)
        if attributes:
            self._numscr.addstr(f'{item}', attributes)
        else:
            self._numscr.addstr(f'{item}')

        height = int(round(item / self._maximum * self.BAR_HEIGHT, 0))
        for i in range(height):
            self._barscr.move(self.BAR_HEIGHT - i - 1,
                              index * self.WIDTH_PER_ITEM)
            if attributes:
                self._barscr.addstr(" ", attributes | self.BAR_ATTRIBUTES)
            else:
                self._barscr.addstr(" ", self.BAR_ATTRIBUTES)

    def draw(self, compared_indexes=[], sorted_indexes=[]):
        self._last_compared_indexes = compared_indexes
        self._last_sorted_indexes = sorted_indexes

        self._numscr.clear()
        self._barscr.clear()
        for (i, item) in enumerate(self._items):
            if i in compared_indexes:
                self._draw_item(i, item, self.COMPARED_ATTRIBUTES)
            elif i in sorted_indexes:
                self._draw_item(i, item, self.SORTED_ATTRIBUTES)
            else:
                self._draw_item(i, item)
        self._numscr.refresh()
        self._barscr.refresh()

    def pause(self):
        self.draw(self._last_compared_indexes, self._last_sorted_indexes)
        self._stdscr.getch()
