//import org.junit.Test
//
//import java.util.Random
//import java.util.UUID
//
//import org.hamcrest.CoreMatchers.`is`
//import org.junit.Assert.assertThat
//import java.lang.StringBuilder
//import kotlin.reflect.jvm.internal.impl.resolve.constants.UByteValue
//
//class BrainLuckTest {
//    private val random = Random(System.currentTimeMillis())
//
//    @Test
//    fun testEchoUntilByte255Encountered() {
//        assertThat(BrainLuck(",+[-.,+]").process("Codewars" + 255.toChar()), `is`("Codewars"))
//    }
//
//    @Test
//    fun testEchoUntilByte0Encountered() {
//        assertThat(BrainLuck(",[.[-],]").process("Codewars" + 0.toChar()), `is`("Codewars"))
//    }
//
//    @Test
//    fun testTwoNumbersMultiplier() {
//        val input = charArrayOf(8.toChar(), 9.toChar())
//        assertThat(BrainLuck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.").process(input[0].toString() + input[1].toString()),
//                `is`<String>((input[0].toInt() * input[1].toInt()).toChar().toString()))
//    }
//
//    @Test
//    fun testFibonacci() {
//        assertThat(BrainLuck("+>+>,-[-[->+<]<<[->>>>+>+<<<<<]>[->>>>>+>+<<<<<<]>>>[-<<<<+>>>>]>[-<<<+>>>]>[-<<<<<+>>>>>]>[-<<<<<+>>>>>]<<<<]")
//                .process(""), `is`("s"))
//    }
//}
//
//class BrainLuck(private val code: String) {
//    fun overflow(num: Int): Int {
//        if (num > 255) return num - 256
//        if (num < 0) return 256 + num
//        return num
//    }
//
//    fun process(input: String): String {
//        val data = mutableMapOf<Int, Int>()
//        var dPointer = 0
//        var iPointer = 0
//        var inpPointer = 0
//        val res = StringBuilder()
//        loop@ while (iPointer < code.length) {
//            when (code[iPointer]) {
//                '>' -> dPointer++
//                '<' -> dPointer--
//                '+' -> data[dPointer] = overflow((data[dPointer] ?: 0) + 1)
//                '-' -> data[dPointer] = overflow((data[dPointer] ?: 0) - 1)
//                '.' -> {
//                    if (data[dPointer] != null) {
//                        res.append(data[dPointer]?.toChar())
//                    }
//                }
//                ',' -> data[dPointer] = input[inpPointer++].toInt()
//                '[' -> {
//                    if (data[dPointer] == 0) {
//                        var openCount = 1
//                        do {
//                            iPointer++
//                            if (code[iPointer] == ']') openCount--
//                            if (code[iPointer] == '[') openCount++
//                        } while (openCount > 0)
//                    }
//                }
//                ']' -> {
//                    if (data[dPointer] != 0) {
//                        var closeCount = 1
//                        do {
//                            iPointer--
//                            if (code[iPointer] == ']') closeCount++
//                            if (code[iPointer] == '[') closeCount--
//                        } while (closeCount > 0)
//                    }
//                }
//            }
//            iPointer++
//        }
//        return res.toString()
//    }
//}