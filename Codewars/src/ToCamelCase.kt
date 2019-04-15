package tocamelcase

import kotlin.test.assertEquals
import org.junit.Test

class TestExample {
    @Test
    fun testFixed() {
        assertEquals("", toCamelCase(""))
        assertEquals("theStealthWarrior", toCamelCase("the_stealth_warrior"))
        assertEquals("TheStealthWarrior", toCamelCase("The-Stealth-Warrior"))
        assertEquals("ABC", toCamelCase("A-B-C"))
    }
}

fun toCamelCase(str:String):String {
    var toUpCaseNext = false
    var res = ""
    for (c in str) {
        val specialChar = (c in arrayOf('-', '_'))
        if (specialChar) {
            toUpCaseNext = true
            continue
        }
        if (toUpCaseNext) {
           res += c.toUpperCase()
        } else {
            res += c
        }
        toUpCaseNext = false
    }
    return res
}