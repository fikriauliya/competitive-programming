import curses
from time import sleep


class Animator:
    WIDTH_PER_ITEM = 5
    BAR_HEIGHT = 20

    def __init__(self, stdscr, y_offset=0):
        self._stdscr = stdscr
        self._statusscr = curses.newwin(1, curses.COLS, 0 + y_offset, 0)
        self._numscr = curses.newwin(1 + self.BAR_HEIGHT, curses.COLS,
                                     2 + y_offset, 0)
        self.COMPARED_ATTRIBUTES = curses.A_UNDERLINE | curses.color_pair(1)
        self.SORTED_ATTRIBUTES = curses.A_BOLD | curses.color_pair(2)
        self.LEFT_ATTRIBUTES = curses.A_BOLD | curses.color_pair(3)
        self.RIGHT_ATTRIBUTES = curses.A_BOLD | curses.color_pair(4)
        self.BAR_ATTRIBUTES = curses.A_REVERSE

    def set_label(self, label):
        self._statusscr.clear()
        self._statusscr.addstr(label)
        self._statusscr.refresh()

    def _draw_item(self, index, item, attributes=None, maximum=10):
        if item is None: return

        self._numscr.move(0, index * self.WIDTH_PER_ITEM)
        if attributes:
            self._numscr.addstr(f'{item}', attributes)
        else:
            self._numscr.addstr(f'{item}')

        height = int(round(item / maximum * self.BAR_HEIGHT, 0))
        for i in range(height):
            self._numscr.move(self.BAR_HEIGHT - i, index * self.WIDTH_PER_ITEM)
            if attributes:
                self._numscr.addstr(" ", attributes | self.BAR_ATTRIBUTES)
            else:
                self._numscr.addstr(" ", self.BAR_ATTRIBUTES)

    def draw(self,
             items,
             compared_indexes=[],
             sorted_indexes=[],
             left_indexes=[],
             right_indexes=[]):
        non_none_items = list(filter(lambda x: x is not None, items))
        if len(non_none_items) == 0:
            self._numscr.clear()
            self._numscr.refresh()
            return
        maximum = max(non_none_items)

        self._numscr.clear()
        for (i, item) in enumerate(items):
            attributes = None
            if i in compared_indexes: attributes = self.COMPARED_ATTRIBUTES
            if i in sorted_indexes: attributes = self.SORTED_ATTRIBUTES
            if i in left_indexes: attributes = self.LEFT_ATTRIBUTES
            elif i in right_indexes: attributes = self.RIGHT_ATTRIBUTES

            if attributes is None: self._draw_item(i, item)
            else: self._draw_item(i, item, maximum=10, attributes=attributes)
        self._numscr.refresh()

    def clear(self):
        self._statusscr.clear()
        self._numscr.clear()
        self._statusscr.refresh()
        self._numscr.refresh()

    def pause(self):
        # self.draw(self._last_compared_indexes, self._last_sorted_indexes)
        # sleep(0.4)
        self._stdscr.getch()