import java.io.InputStreamReader
import java.io.BufferedReader
import java.lang.StringBuilder
import java.util.Scanner

fun main(args: Array<String>) {
    val input = Scanner(BufferedReader(InputStreamReader(System.`in`)))
    val t = input.nextInt() // Scanner has functions to read ints, longs, strings, chars, etc.
    for (i in 1..t) {
        val size = input.nextInt()
        val p = input.next()
        val res = StringBuilder()
        for (c in p) {
            res.append(if (c == 'S') 'E' else 'S')
        }
        println("Case #$i: ${res.toString()}")
    }
}