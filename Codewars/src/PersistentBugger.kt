/**
 * Created by levi on 26/01/19.
 */
import kotlin.test.assertEquals
import org.junit.Test

class TestPersistence {
    @Test
    fun `Basic Tests`() {
        assertEquals(3, persistence(39))
        assertEquals(0, persistence(4))
        assertEquals(2, persistence(25))
        assertEquals(4, persistence(999))
    }
}

fun persistence(num: Int) : Int {
    val digits = Math.floor(Math.log10(num.toDouble())) + 1
    if (digits <= 1) return 0
    var mult = 1
    var rem = num
    while (rem > 0) {
        mult *= (rem % 10)
        rem /= 10
    }
    return 1 + persistence(mult)
}

