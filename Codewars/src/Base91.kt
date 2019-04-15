import org.junit.Test

import java.util.Random

import org.junit.Assert.assertEquals

class Base91Test {

    @Test
    fun fixedTests() {
        assertEquals(Base91.encode("test"), "fPNKd")
        assertEquals(Base91.encode("Hello World!"), ">OwJh>Io0Tv!8PE")
        assertEquals(Base91.decode("fPNKd"), "test")
        assertEquals(Base91.decode(">OwJh>Io0Tv!8PE"), "Hello World!")
    }

}
object Base91 {
    val encoding = """ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!#$%&()*+,./:;<=>?@[]^_`{|}~""""
    fun encode(data: String): String {
        return String()
    }
    fun decode(data: String): String {
        return String() // do it!
    }
}