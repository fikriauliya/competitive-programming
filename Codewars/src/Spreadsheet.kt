package spreadsheet

import kotlin.test.assertEquals
import org.junit.Test

class TestExample {
    @Test
    fun testFixed() {
        assertEquals("R1C1", spreadsheet("A1"))
        assertEquals("A1", spreadsheet("R1C1"))
        assertEquals("D5", spreadsheet("R5C4"))
        assertEquals("R48C27", spreadsheet("AA48"))
        assertEquals("R12C63", spreadsheet("BK12"))
        assertEquals("BK12", spreadsheet("R12C63"))
        assertEquals("Z85", spreadsheet("R85C26"))
        assertEquals("BZ31", spreadsheet("R31C78"))
        assertEquals("R31C78", spreadsheet("BZ31"))
    }
}

fun toBase26(num: Int): String {
    if (num <= 26) return ('A'.toInt() + num - 1).toChar().toString()
    return toBase26((num - 1) / 26) + toBase26((num - 1) % 26 + 1)
}

fun fromBase26(s: String): Int {
    if (s.isEmpty()) return 0
    if (s.length == 1) return s[0] - 'A' + 1
    return fromBase26(s.substring(0, s.length - 1)) * 26 + fromBase26(s[s.length-1].toString())
}

fun spreadsheet(s: String): String {
    val rc = Regex("""R(\d+)C(\d+)""").matchEntire(s)
    var res = ""
    if (rc != null) {
        val row = rc.groupValues[1]
        val col = rc.groupValues[2].toInt()
        res = "${toBase26(col)}$row"
    }
    val sp = Regex("""(\D+)(\d+)""").matchEntire(s)
    if (sp != null) {
        val col = sp.groupValues[1]
        val row = sp.groupValues[2]
        res = "R${row}C${fromBase26(col)}"
    }
    return res
}