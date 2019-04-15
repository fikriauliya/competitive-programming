import org.junit.Test
import org.junit.Assert.*

class TimeFormatterTest {
    @Test
    fun testFormatDurationExamples() {
        // Example Test Cases
        assertEquals("", TimeFormatter.formatDuration(0))
        assertEquals("1 second", TimeFormatter.formatDuration(1))
        assertEquals("1 minute and 2 seconds", TimeFormatter.formatDuration(62))
        assertEquals("2 minutes", TimeFormatter.formatDuration(120))
        assertEquals("1 hour", TimeFormatter.formatDuration(3600))
        assertEquals("1 hour, 1 minute and 2 seconds", TimeFormatter.formatDuration(3662))
    }
}

object TimeFormatter {
    fun formatDuration(s: Int): String {
        var seconds = s
        var minutes = seconds / 60
        var hours = minutes / 60
        var days = hours / 24
        val years = days / 365
        days %= 365
        hours %= 24
        minutes %= 60
        seconds %= 60
        val nums = intArrayOf(years, days, hours, minutes, seconds)
        val labels = arrayOf("year", "day", "hour", "minute", "second")
        val pairs = nums.zip(labels)
                .filter { (n, _) -> n > 0 }
                .map { (n, l) -> if (n > 1) "$n ${l}s" else "$n $l" }
        if (pairs.isEmpty()) return "now"

        var res = ""
        for (i in 0 until pairs.size - 1) {
            res += if (i == pairs.size - 2) "${pairs[i]} and "
            else "${pairs[i]}, "
        }
        res += pairs.last()
        return res
    }
}