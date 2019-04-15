package stringincrementer

import kotlin.test.assertEquals
import org.junit.Test

class TestExample {
    @Test
    fun FixedTests() {
        assertEquals(incrementString("foobar000"), "foobar001")
        assertEquals(incrementString("foobar999"), "foobar1000")
        assertEquals(incrementString("foobar00999"), "foobar01000")
        assertEquals(incrementString("foo"), "foo1")
        assertEquals(incrementString("foobar001"), "foobar002")
        assertEquals(incrementString("foobar1"), "foobar2")
        assertEquals(incrementString("1"), "2")
        assertEquals(incrementString(""), "1")
        assertEquals(incrementString("009"), "010")
    }
}

fun incrementString(str: String) : String {
    val m = Regex("""^(.*?)(\d+)$""").matchEntire(str)
    if (m != null) {
        val num = (m.groupValues[2].toInt() + 1).toString()
        val rem = m.groupValues[2].length - num.length
        val zeroPrefix = if (rem > 0) "0".repeat(rem) else ""
        return "${m.groupValues[1]}$zeroPrefix$num"
    }
    return str + "1"
}