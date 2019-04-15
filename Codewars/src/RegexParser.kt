//import org.junit.Assert.assertEquals
//import org.junit.Test
//import java.util.*
//import java.util.stream.Collectors
//import java.util.stream.IntStream
//import kotlin.test.expect
//
//class RegExpTest {
//    /* **************** *
//     *  RANDOM TESTS  *
//     * ************** */
//
//    private val rand = Random()
//    private val randStr: String
//        get() {
//            val sb = StringBuilder(10 + rand.nextInt(100))
//            for (i in 0 until sb.capacity())
//                sb.append((32 + rand.nextInt(94)).toChar())
//            return sb.toString()
//        }
//
//    @Test
//    fun coreDisplay___NOT_A_TEST___Look_Inside___() {
//        println(Normal('a'))
//        println(ZeroOrMore(Normal('a')))
//        println(Or(Any(), ZeroOrMore(Normal('a'))))
//        println(Str(listOf(Normal('a'), Normal('b'))))
//        println(Str(listOf(Normal('b'),
//                Or(Normal('c'),
//                        Normal('d')),
//                ZeroOrMore(Normal('e')))))
//    }
//
//    private fun shouldBe(input: String, expected: String) {
//        val actual = RegExpParser(input).parse()
//        assertEquals(String.format("Parsing \"%s\": ", input), expected, "$actual")
//    }
//
//
//    @Test
//    fun basicTests() {
//        for (s in listOf(arrayOf(".", "."),
//                arrayOf("a", "a"),
//                arrayOf("a|b", "(a|b)"),
//                arrayOf("a*", "a*"),
//                arrayOf("(a)", "a"),
//                arrayOf("(a)*", "a*"),
//                arrayOf("(a|b)*", "(a|b)*"),
//                arrayOf("a|b*", "(a|b*)"),
//                arrayOf("abcd", "(abcd)"),
//                arrayOf("ab|cd", "((ab)|(cd))"))) shouldBe(s[0], s[1])
//    }
//
//
//    @Test
//    fun precedenceTests() {
//        for (s in listOf(arrayOf("ab*", "(ab*)"),
//                arrayOf("(ab)*", "(ab)*"),
//                arrayOf("ab|a", "((ab)|a)"),
//                arrayOf("a(b|a)", "(a(b|a))"),
//                arrayOf("a|b*", "(a|b*)"),
//                arrayOf("(a|b)*", "(a|b)*"))) shouldBe(s[0], s[1])
//    }
//
//
//    @Test
//    fun otherExamples() {
//        for (s in listOf(arrayOf("a", "a"),
//                arrayOf("ab", "(ab)"),
//                arrayOf("a.*", "(a.*)"),
//                arrayOf("(a.*)|(bb)", "((a.*)|(bb))"))) shouldBe(s[0], s[1])
//    }
//
//
//    @Test
//    fun invalidTests() {
//        for (s in listOf(arrayOf("*", ""),
//                arrayOf("(", ""),
//                arrayOf("(hi!", ""),
//                arrayOf(")(", ""),
//                arrayOf("a|t|y", ""),
//                arrayOf("a**", ""))) shouldBe(s[0], s[1])
//    }
//
//    @Test
//    fun complexExamples() {
//        for (s in listOf(arrayOf("((aa)|ab)*|a", "(((aa)|(ab))*|a)"),
//                arrayOf("((a.)|.b)*|a", "(((a.)|(.b))*|a)"))) shouldBe(s[0], s[1])
//    }
//}
//
//sealed class RegExp
//data class Normal(val s:Char): RegExp()
//data class ZeroOrMore(val v:RegExp): RegExp()
//data class Or(val v1:RegExp, val v2: RegExp): RegExp()
//class Any(): RegExp()
//data class Str(val vs: List<RegExp>): RegExp()
//
//class UtilityTest {
//    @Test
//    fun removeParanthesisTest() {
//        assertEquals("", RegExpParser.removeParenthesis("(abc)"))
//        assertEquals("b", RegExpParser.removeParenthesis("(a)b"))
//        assertEquals("b", RegExpParser.removeParenthesis("b(a)"))
//        assertEquals("bc", RegExpParser.removeParenthesis("(a)bc"))
//        assertEquals(null, RegExpParser.removeParenthesis("((abc)"))
//        assertEquals(null, RegExpParser.removeParenthesis("(abc))"))
//        assertEquals(null, RegExpParser.removeParenthesis("((abc)a"))
//        assertEquals(null, RegExpParser.removeParenthesis("(abc))a"))
//    }
//}
//class RegExpParser(val input: String) {
//    companion object {
//        fun removeParenthesis(inp : String): String? {
//            // (abc) ->
//            // (a)b -> b
//            // b(a) -> b
//            // ((abc) ->
//            // (abc)) ->
//            // (abc))a ->
//            val s = Stack<Char>()
//            for (c in inp) {
//                when (c) {
//                    ')' -> {
//                        var found = false
//                        while (!s.empty()) {
//                            found = s.pop() == '('
//                            if (found) break
//                        }
//                        if (!found) return null
//                    }
//                    else -> s.push(c)
//                }
//            }
//            if (s.empty()) return ""
//            val res = s.joinToString("")
//            if (res.contains("(")) return null
//            return res
//        }
//    }
//
//    fun parse(): RegExp {
//        fun isOrExp(inp: String): Boolean {
//            // Yes: a|b, (a)|b, ab|c, (ab)|c, a*|b, (a*)|b
//            // No: (a|b)c, a|b|c
//            return removeParenthesis(inp)?.contains("|") ?: false
//        }
//        fun parseRec(inp: String): RegExp {
//            // Operator precedences from highest to lowest are: *, sequencing and |. Therefore the followings hold:
//            when {
//                inp.startsWith("(") && inp.endsWith(")") -> parseRec(inp.substring(1, inp.length - 1))
//                inp.length == 1 && inp[0] in 'a'..'z' -> Normal(inp[0])
//                isOrExp(inp) -> Or()
//                inp.length == 2 && inp.endsWith("*") -> ZeroOrMore(parseRec(inp.substring(0, 1)))
//                inp.length == 3 && inp[1] == '|' -> Or(parseRec(inp.s))
//                //s*, s*d, as*, (a)s*, (as)*
//            }
//            return Normal('s')
//        }
//        return parseRec(input)
//    }
//}