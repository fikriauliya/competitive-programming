
import java.io.InputStreamReader
import java.io.BufferedReader
import java.util.Scanner

fun main(args: Array<String>) {
    val input = Scanner(BufferedReader(InputStreamReader(System.`in`)))
    val t = input.nextInt() // Scanner has functions to read ints, longs, strings, chars, etc.
    for (i in 1..t) {
        val n = input.next()
        val a = n.replace('4', '2')
        val b = n
                .map { if (it == '4') '4' else '0'  }
                .joinToString("")
                .replace('4', '2')
                .toBigInteger().toString()
        println("Case #$i: $a $b")
    }
}