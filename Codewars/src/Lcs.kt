import com.sun.xml.internal.fastinfoset.util.StringArray
import org.junit.Test

import java.util.Arrays
import java.util.Random

import org.junit.Assert.*

class LcsTest {

    @Test
    fun fixedTests() {
        assertEquals("", lcs("", ""))
        assertEquals("", lcs("abc", ""))
        assertEquals("", lcs("", "abc"))
        assertEquals("", lcs("a", "b"))
        assertEquals("a", lcs("a", "a"))
        assertEquals("ac", lcs("abc", "ac"))
        assertEquals("abc", lcs("abcdef", "abc"))
        assertEquals("acf", lcs("abcdef", "acf"))
        assertEquals("nottest", lcs("anothertest", "notatest"))
        assertEquals("12356", lcs("132535365", "123456789"))
        assertEquals("final", lcs("nothardlythefinaltest", "zzzfinallyzzz"))
        assertEquals("acdefghijklmnoq", lcs("abcdefghijklmnopq", "apcdefghijklmnobq"))
    }

}

fun lcs(a: String, b: String): String {
    val mem = mutableMapOf<Pair<String, String>, String>()

    fun lcsRec(a: String, b:String): String {
        if (a == "" || b == "") return ""
        if (mem[Pair(a, b)] != null) return mem[Pair(a, b)]!!

        val s1 = lcsRec(a.substring(1), b)
        val s2 = lcsRec(a, b.substring(startIndex = 1))

        val res = if (a[0] == b[0]) {
            val s3 = a[0] + lcsRec(a.substring(1), b.substring(1))
            arrayOf(s1, s2, s3).maxBy { it -> it.length } ?: ""
        } else {
            arrayOf(s1, s2).maxBy { it -> it.length } ?: ""
        }
        mem[Pair(a, b)] = res
        return res
    }
    return lcsRec(a, b)
}
