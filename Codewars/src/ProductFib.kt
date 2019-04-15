package prodfib

import org.junit.Assert.*
import org.junit.Test

class ProdFibTest {
    @Test
    fun test1() {
        val r = longArrayOf(55, 89, 1)
        assertArrayEquals(r, productFib(4895))
    }
    @Test
    fun test2() {
        val r = longArrayOf(89, 144, 0)
        assertArrayEquals(r, productFib(5895))
    }

}

fun productFib(prod:Long):LongArray {
    var fib0 = 0L
    var fib1 = 1L
    while (fib0 * fib1 < prod) {
        val fib2= fib0 + fib1
        fib0 = fib1
        fib1 = fib2
    }
    if (fib0 * fib1 == prod) {
        return longArrayOf(fib0, fib1, 1)
    }
    return longArrayOf(fib0, fib1, 0)
}