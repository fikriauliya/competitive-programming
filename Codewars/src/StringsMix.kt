package mix

import org.junit.Assert.*
import org.junit.Test

class mixinMainTest {
    @Test
    fun test() {
        println("Fixed Tests")
        assertEquals("2:eeeee/2:yy/=:hh/=:rr", mix("Are they here", "yes, they are here"))
        assertEquals("1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg",
                mix("looping is fun but dangerous", "less dangerous than coding"))

    }

}

fun mix(s1: String, s2: String): String {
    val ss = arrayOf(s1, s2)
    val cMaps = ss
            .map {
                it
                        .groupBy { c -> c }
                        .filterKeys { k -> k in 'a'..'z' }
                        .mapValues { (_, v) -> v.size }
            }
    val cJoin = ('a'..'z')
            .map { c: Char ->
                val counts = cMaps.map { it[c] ?: 0 }
                val maxCounts = counts.max() ?: 0
                if (maxCounts <= 1) return@map Pair("", "")
                val repeated = c.toString().repeat(maxCounts)
                if (counts[0] == counts[1]) {
                    Pair("=", repeated)
                } else {
                    val winner = if (counts[0] == maxCounts) 1 else 2
                    Pair(winner.toString(), repeated)
                }
            }
            .filter { (k, _) -> k != "" }
            .sortedWith(
                    Comparator { (k1, v1), (k2, v2) ->
                        val c1 = v1.length.compareTo(v2.length)
                        if (c1 != 0) return@Comparator -c1
                        "$k1:$v1".compareTo("$k2:$v2")
                    }
            )
            .map{(k, v) -> "$k:$v"}
            .joinToString(separator = "/")
    return cJoin
}
