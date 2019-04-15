import org.junit.Assert.*

class TestExample {
    @org.junit.Test
    @Throws(Exception::class)
    fun basicTest() {
        assertEquals(86, predictAge(65, 60, 75, 55, 60, 63, 64, 45))
        assertEquals(79, predictAge(32, 54, 76, 65, 34, 63, 64, 45))
    }
}

fun predictAge(age1: Int, age2: Int, age3: Int, age4: Int, age5: Int, age6: Int, age7: Int, age8: Int): Int{
    val sum = (age1 * age1) + (age2 * age2) + (age3 * age3) + (age4 * age4) + (age5 * age5) + (age6 * age6) + (age7 * age7) + (age8 * age8)
    return Math.floor(Math.sqrt(sum.toDouble()) / 2.0).toInt()
}