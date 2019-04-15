//import org.junit.Test
//import org.junit.Assert.*
//import org.junit.runners.JUnit4
//
//class MineSweeperTests {
//    /* Data structure for one test:
//     *      0 -> name or comment of the test
//     *      1 -> original map
//     *      2 -> map sent to the user
//     *      3 -> expected solution
//     */
//    private val DATA = arrayOf(// Sample Tests:
//            arrayOf("Simple map 1",
//                    """1 x 1 1 x 1
//                      |2 2 2 1 2 2
//                      |2 x 2 0 1 x
//                      |2 x 2 1 2 2
//                      |1 1 1 1 x 1
//                      |0 0 0 1 1 1""".trimMargin(),
//                    """? ? ? ? ? ?
//                      |? ? ? ? ? ?
//                      |? ? ? 0 ? ?
//                      |? ? ? ? ? ?
//                      |? ? ? ? ? ?
//                      |0 0 0 ? ? ?""".trimMargin(),
//                    """1 x 1 1 x 1
//                      |2 2 2 1 2 2
//                      |2 x 2 0 1 x
//                      |2 x 2 1 2 2
//                      |1 1 1 1 x 1
//                      |0 0 0 1 1 1""".trimMargin()),
//
//            arrayOf("Simple map 2",
//                    """0 2 x
//                      |0 2 x""".trimMargin(),
//                    """0 ? ?
//                      |0 ? ?""".trimMargin(),
//                    """0 2 x
//                      |0 2 x""".trimMargin()),
//
//            arrayOf("Simple unsolvable map",
//                    """0 1 x
//                      |0 1 1""".trimMargin(),
//                    """0 ? ?
//                      |0 ? ?""".trimMargin(),
//                    "?"),
//
//            arrayOf("Simple map 3",
//                    """1 x x 1 0 0 0
//                      |2 3 3 1 0 1 1
//                      |1 x 1 0 0 1 x
//                      |1 1 1 0 0 1 1
//                      |0 1 1 1 0 0 0
//                      |0 1 x 1 0 0 0
//                      |0 1 1 1 0 1 1
//                      |0 0 0 0 0 1 x
//                      |0 0 0 0 0 1 1""".trimMargin(),
//                    """? ? ? ? 0 0 0
//                      |? ? ? ? 0 ? ?
//                      |? ? ? 0 0 ? ?
//                      |? ? ? 0 0 ? ?
//                      |0 ? ? ? 0 0 0
//                      |0 ? ? ? 0 0 0
//                      |0 ? ? ? 0 ? ?
//                      |0 0 0 0 0 ? ?
//                      |0 0 0 0 0 ? ?""".trimMargin(),
//                    """1 x x 1 0 0 0
//                      |2 3 3 1 0 1 1
//                      |1 x 1 0 0 1 x
//                      |1 1 1 0 0 1 1
//                      |0 1 1 1 0 0 0
//                      |0 1 x 1 0 0 0
//                      |0 1 1 1 0 1 1
//                      |0 0 0 0 0 1 x
//                      |0 0 0 0 0 1 1""".trimMargin()),
//
//            arrayOf("Various unsolvable map - 1",
//                    """1 1 0 1 1 1 0 0 1 1 1 0 0 0 0 1 1 1 0
//                      |x 1 0 1 x 1 0 0 2 x 2 0 0 0 0 1 x 2 1
//                      |1 1 0 2 3 3 1 1 3 x 2 0 0 0 0 1 2 x 1
//                      |0 1 1 2 x x 1 2 x 3 1 0 0 0 0 0 1 1 1
//                      |0 1 x 2 2 2 1 3 x 3 0 0 0 0 0 0 0 0 0
//                      |0 1 1 1 0 0 0 2 x 2 0 0 0 0 0 0 0 0 0
//                      |0 0 0 0 0 0 0 1 1 1 1 2 2 1 0 0 0 0 0
//                      |0 0 0 0 0 0 0 0 0 0 1 x x 1 0 0 0 0 0
//                      |0 0 1 1 1 0 1 1 1 0 1 2 2 1 0 0 0 0 0
//                      |0 0 1 x 2 1 3 x 2 0 0 0 0 0 0 1 1 1 0
//                      |0 0 1 1 2 x 3 x 3 1 1 0 0 0 0 1 x 1 0
//                      |0 0 0 0 1 2 3 2 2 x 1 0 0 0 0 1 1 1 0
//                      |0 0 0 0 0 1 x 1 1 1 1 0 0 0 0 0 1 1 1
//                      |0 0 1 1 2 2 2 1 0 0 0 0 0 0 0 0 1 x 1
//                      |0 0 1 x 2 x 2 1 1 0 0 0 0 0 0 0 1 1 1
//                      |0 0 1 1 2 1 3 x 3 1 0 0 0 0 0 0 0 1 1
//                      |0 0 0 0 0 0 2 x x 1 0 0 0 1 1 1 0 1 x
//                      |0 0 0 1 1 1 1 2 2 1 0 0 0 1 x 1 1 2 2
//                      |0 0 0 1 x 3 2 1 0 0 0 1 1 2 1 1 1 x 2
//                      |0 0 0 1 2 x x 1 0 0 0 1 x 1 0 1 2 3 x
//                      |0 0 0 0 1 2 2 1 1 1 1 1 1 1 0 1 x 3 2
//                      |0 0 0 0 1 1 1 1 2 x 1 1 1 1 0 2 3 x 2
//                      |0 0 0 0 1 x 1 1 x 2 1 1 x 1 0 1 x 3 x""".trimMargin(),
//                    """? ? 0 ? ? ? 0 0 ? ? ? 0 0 0 0 ? ? ? 0
//                      |? ? 0 ? ? ? 0 0 ? ? ? 0 0 0 0 ? ? ? ?
//                      |? ? 0 ? ? ? ? ? ? ? ? 0 0 0 0 ? ? ? ?
//                      |0 ? ? ? ? ? ? ? ? ? ? 0 0 0 0 0 ? ? ?
//                      |0 ? ? ? ? ? ? ? ? ? 0 0 0 0 0 0 0 0 0
//                      |0 ? ? ? 0 0 0 ? ? ? 0 0 0 0 0 0 0 0 0
//                      |0 0 0 0 0 0 0 ? ? ? ? ? ? ? 0 0 0 0 0
//                      |0 0 0 0 0 0 0 0 0 0 ? ? ? ? 0 0 0 0 0
//                      |0 0 ? ? ? 0 ? ? ? 0 ? ? ? ? 0 0 0 0 0
//                      |0 0 ? ? ? ? ? ? ? 0 0 0 0 0 0 ? ? ? 0
//                      |0 0 ? ? ? ? ? ? ? ? ? 0 0 0 0 ? ? ? 0
//                      |0 0 0 0 ? ? ? ? ? ? ? 0 0 0 0 ? ? ? 0
//                      |0 0 0 0 0 ? ? ? ? ? ? 0 0 0 0 0 ? ? ?
//                      |0 0 ? ? ? ? ? ? 0 0 0 0 0 0 0 0 ? ? ?
//                      |0 0 ? ? ? ? ? ? ? 0 0 0 0 0 0 0 ? ? ?
//                      |0 0 ? ? ? ? ? ? ? ? 0 0 0 0 0 0 0 ? ?
//                      |0 0 0 0 0 0 ? ? ? ? 0 0 0 ? ? ? 0 ? ?
//                      |0 0 0 ? ? ? ? ? ? ? 0 0 0 ? ? ? ? ? ?
//                      |0 0 0 ? ? ? ? ? 0 0 0 ? ? ? ? ? ? ? ?
//                      |0 0 0 ? ? ? ? ? 0 0 0 ? ? ? 0 ? ? ? ?
//                      |0 0 0 0 ? ? ? ? ? ? ? ? ? ? 0 ? ? ? ?
//                      |0 0 0 0 ? ? ? ? ? ? ? ? ? ? 0 ? ? ? ?
//                      |0 0 0 0 ? ? ? ? ? ? ? ? ? ? 0 ? ? ? ?""".trimMargin(),
//                    "?"),
//
//            arrayOf("Various unsolvable map - 2",
//                    """0 0 0 0 0 0 0 0 1 x x 2 1 0 1 x 1 0 1 2 x
//                      |0 0 0 0 0 0 0 0 1 2 3 x 1 0 2 2 2 1 2 x 2
//                      |0 0 0 0 0 0 0 0 0 0 2 2 2 0 1 x 1 1 x 2 1
//                      |0 0 0 0 0 1 1 1 0 0 1 x 1 0 1 2 2 2 1 1 0
//                      |1 1 0 0 0 1 x 1 0 1 2 2 1 0 0 1 x 1 1 1 1
//                      |x 1 0 0 0 1 1 1 0 1 x 1 0 0 0 1 1 1 1 x 1
//                      |2 2 1 0 0 0 0 0 0 1 1 1 0 0 0 0 0 0 1 1 1
//                      |1 x 1 0 0 0 0 0 0 0 1 2 2 1 0 0 1 1 1 0 0
//                      |1 1 1 0 0 0 0 0 0 0 1 x x 1 0 0 1 x 1 0 0""".trimMargin(),
//                    """0 0 0 0 0 0 0 0 ? ? ? ? ? 0 ? ? ? 0 ? ? ?
//                      |0 0 0 0 0 0 0 0 ? ? ? ? ? 0 ? ? ? ? ? ? ?
//                      |0 0 0 0 0 0 0 0 0 0 ? ? ? 0 ? ? ? ? ? ? ?
//                      |0 0 0 0 0 ? ? ? 0 0 ? ? ? 0 ? ? ? ? ? ? 0
//                      |? ? 0 0 0 ? ? ? 0 ? ? ? ? 0 0 ? ? ? ? ? ?
//                      |? ? 0 0 0 ? ? ? 0 ? ? ? 0 0 0 ? ? ? ? ? ?
//                      |? ? ? 0 0 0 0 0 0 ? ? ? 0 0 0 0 0 0 ? ? ?
//                      |? ? ? 0 0 0 0 0 0 0 ? ? ? ? 0 0 ? ? ? 0 0
//                      |? ? ? 0 0 0 0 0 0 0 ? ? ? ? 0 0 ? ? ? 0 0""".trimMargin(),
//                    "?"))
//
//
//    private fun makeAssertionAndDisplay(expected: String, actual: String) {
//        makeAssertionAndDisplay("", expected, actual)
//    }
//
//    private fun makeAssertionAndDisplay(message: String, expected: String, actual: String) {
//        if (expected != actual) {
//            println("""${if (message.isEmpty()) "Failed test!!" else message}
//
//                |Expected:
//                |$expected
//
//                |But was:
//                |$actual""".trimMargin())
//        }
//        assertEquals(message, expected, actual)
//    }
//
//
//    @Test
//    fun sampleTests() {
//        for (count in 0..5) {
//            Game.newGame(DATA[count][1])
//            Game.read(DATA[count][2])
//            makeAssertionAndDisplay(DATA[count][0], DATA[count][3], MineSweeper(DATA[count][2], Game.minesN).solve())
//        }
//    }
//}
//
//class MineSweeper(board: String, nMines: Int) {
//    init {
//
//    }
//
//    fun solve(): String {
//        return ""
//    }
//}