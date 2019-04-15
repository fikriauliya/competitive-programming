import java.io.InputStreamReader
import java.io.BufferedReader
import java.lang.StringBuilder
import java.math.BigInteger
import java.util.*

fun gcd(n: BigInteger, m: BigInteger): BigInteger {
    if (m == BigInteger.ZERO) {
        return n
    }
    return gcd(m, n.mod(m))
}
fun main(args: Array<String>) {
    val input = Scanner(BufferedReader(InputStreamReader(System.`in`)))
    val t = input.nextInt()
    for (i in 1..t) {
        val n = BigInteger(input.next())
        val l = input.nextInt()
        val ciphers = Array<BigInteger>(l) { BigInteger.ZERO }
        for (j in 0 until l) {
            ciphers[j] = BigInteger(input.next())
        }
        val primes = LinkedList<BigInteger>()

        var breakPoint = 0
        var comm: BigInteger? = null
        for (j in 0 until l) {
            comm = gcd(ciphers[j], ciphers[j+1])
            if (comm != 1.toBigInteger()) {
                breakPoint = j
                break;
            }
        }
        var newPrime = comm
        if (comm != null) primes.add(comm)
        for (j in breakPoint downTo 0) {
            newPrime = ciphers[j].divide(newPrime)
            primes.add(0, newPrime)
        }
        newPrime = comm
        for (j in breakPoint + 1 until l) {
            newPrime = ciphers[j].divide(newPrime)
            primes.add(newPrime)
        }
//        println(Arrays.toString(primes.toArray()))
        val sortedPrimes = primes.toSortedSet().toList()
        val map = HashMap<BigInteger, Char>()
        for (j in 0 until sortedPrimes.size) {
            map[sortedPrimes[j]] = 'A' + j
        }
        val res = StringBuilder()
        for (p in primes) {
            res.append(map[p])
        }
        println("Case #$i: $res")
    }
}