package stripcomment

import kotlin.test.assertEquals
import org.junit.Test

class TestExample {
    @Test
    fun testFixed() {
        assertEquals("apples, plums\npears\noranges", solution("apples, plums % and bananas\npears\noranges !applesauce", charArrayOf('%', '!')))
        assertEquals("Q\nu\ne", solution("Q @b\nu\ne -e f g", charArrayOf('@', '-')))
    }
}

fun solution(input: String, markers: CharArray): String {
    return input.split("\n").map{
        inp ->
        var firstMarkerIndex = inp.indexOfFirst { c -> c in markers }
        if (firstMarkerIndex != -1) {
            return@map inp.removeRange(firstMarkerIndex, inp.length).trimEnd()
        }
        return@map inp
    }.joinToString(separator = "\n")
}
