package smallestpossiblesum

import com.sun.source.tree.BinaryTree
import javafx.collections.transformation.SortedList
import kotlin.test.assertEquals
import org.junit.Test

class TestExample {
    @Test
    fun `Basic tests`() {
        assertEquals(9, solution(longArrayOf(6,9,21)))
        assertEquals(3, solution(longArrayOf(1,21,55)))
        assertEquals(5, solution(longArrayOf(3,13,23,7,83)))
        assertEquals(923, solution(longArrayOf(71,71,71,71,71,71,71,71,71,71,71,71,71)))
        assertEquals(22, solution(longArrayOf(11,22)))
        assertEquals(2, solution(longArrayOf(5,17)))
        assertEquals(12, solution(longArrayOf(4,16,24)))
        assertEquals(9, solution(longArrayOf(9)))
    }
}

fun solution(numbers: LongArray): Long {
    var asc = java.util.PriorityQueue<Long>()
    asc.addAll(numbers.toTypedArray())

    do {
//        print(asc)
        var min = asc.poll()
//        println("min: $min")
        var asc2 = java.util.PriorityQueue<Long>()
        asc2.add(min)
        var newMin = asc.poll()
        while (newMin != null) {
            if (newMin > min) {
                if (newMin % min != 0L) newMin -= min * Math.floorDiv(newMin, min)
                else newMin = min
            }
//            println("newMin: $newMin")
            asc2.add(newMin)
            newMin = asc.poll()
        }
        asc = asc2
    } while (asc.peek() != (asc.max() ?: asc.peek()))
    return asc.sum()
}