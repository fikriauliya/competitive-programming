package dbllinear

import org.junit.Assert.*
import org.junit.Test
import org.junit.Assert.*
import kotlin.test.assertEquals

class dblLinearMainTest {
    @Test
    fun test() {
        println("Fixed Tests dblLinear")
        testing(dblLinear(10), 22)
        testing(dblLinear(20), 57)
        testing(dblLinear(30), 91)

    }
    companion object {
        private fun testing(actual:Int, expected:Int) {
            assertEquals(expected.toLong(), actual.toLong())
        }
    }
}

fun dblLinear(n:Int):Int {
    val coll = sortedSetOf(1)
    var ctr = 0
    while (ctr++ < n) {
        val first = coll.first()
        coll.remove(first)
        coll.add(first * 2 + 1)
        coll.add(first * 3 + 1)
    }
    return coll.first()
}

