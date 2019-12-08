import curses
from animator import Animator
from time import sleep
from insertion_sort import InsertionSort
from bubble_sort import BubbleSort
from selection_sort import SelectionSort


def main(stdscr):
    def init_ui():
        curses.init_pair(1, curses.COLOR_RED, curses.COLOR_BLACK)
        curses.init_pair(2, curses.COLOR_CYAN, curses.COLOR_BLACK)
        curses.curs_set(0)

    def simulate():
        sorter_classes = [SelectionSort, BubbleSort, InsertionSort]
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
