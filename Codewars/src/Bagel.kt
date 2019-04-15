import org.junit.Test
import org.junit.Assert

class BagelTest {
    @Test
    fun testBagel() {
        org.junit.Assert.assertEquals((bagel as Bagel).value == 4, java.lang.Boolean.TRUE)
    }
}
class Bagel {
    val value: Int get() = 3
}
val bagel: Bagel
    get() = Bagel() // fix it!

