import java.math.BigInteger;
import org.junit.Test;
import static org.junit.Assert.assertEquals;

public class FibonacciTest {

    @Test
    public void testFib0() {
        testFib(0, 0);
    }

    @Test
    public void testFib1() {
        testFib(1, 1);
    }

    @Test
    public void testFib2() {
        testFib(1, 2);
    }

    @Test
    public void testFib3() {
        testFib(2, 3);
    }

    @Test
    public void testFib4() {
        testFib(3, 4);
    }

    @Test
    public void testFib5() {
        testFib(5, 5);
    }

    @Test
    public void testFib51() {
        testFib(8, 6);
    }

    @Test
    public void testFib52() {
        testFib(13, 7);
    }

    @Test
    public void testFib6() {
        testFib(1, -1);
    }

    @Test
    public void testFib7() {
        testFib(-1, -2);
    }

    @Test
    public void testFib8() {
        testFib(2, -3);
    }

    @Test
    public void testFib9() {
        testFib(100000, 2000000);
    }

    private static void testFib(long expected, long input) {
        BigInteger found;
        try {
            found = Fibonacci.fib(BigInteger.valueOf(input));
        }
        catch (Throwable e) {
            // see https://github.com/Codewars/codewars.com/issues/21
            throw new AssertionError("exception during test: "+e, e);
        }
        assertEquals(BigInteger.valueOf(expected), found);
    }

}

class Fibonacci {
    public static void print(BigInteger[][] a) {
        for (int i=0;i<a.length;i++) {
            for (int j=0;j<a[0].length;j++) {
                System.out.print(a[i][j] + "\t");
            }
            System.out.println();
        }
    }

    public static BigInteger[][] multiply(BigInteger[][] a, BigInteger[][] b) {
        BigInteger[][] res = new BigInteger[a.length][b[0].length];

        for (int i=0; i<a.length; i++) {
            for (int j=0;j<b[0].length; j++) {
                res[i][j] = BigInteger.ZERO;
                for (int k=0;k<b.length;k++) {
                    res[i][j] = res[i][j].add(a[i][k].multiply(b[k][j]));
                }
            }
        }
        return res;
    }

    public static BigInteger[][] pow(BigInteger[][] a, int n) {
        if (n == 1) return a;
        if (n % 2 == 0) {
            BigInteger[][] half = pow(a, n/2);
            return multiply(half, half);
        }
        return multiply(a, pow(a, n-1));
    }


    public static BigInteger fib(BigInteger n) {
        int nInt = n.intValue();
        if (nInt == 0) return new BigInteger("0");

        BigInteger ZERO = BigInteger.ZERO;
        BigInteger ONE = BigInteger.ONE;
        if (nInt < 0) {
            /*
            fib(n) = -fib(n-1) + fib(n-2)
            fib(0) = 0
            fib(1) = 1
            fib(2) = -1
            fib(3) = 2
            [0 1]
            [1 -1]
             */
            BigInteger[][] matrix = {{ZERO, ONE}, {ONE, new BigInteger("-1")}};
            BigInteger[][] init = {{ZERO}, {ONE}};
            BigInteger[][] res = multiply(pow(matrix, -nInt), init);

            return res[0][0];
        } else {
            BigInteger[][] matrix = {{ZERO, ONE}, {ONE, ONE}};
            BigInteger[][] init = {{ZERO}, {ONE}};
            BigInteger[][] res = multiply(pow(matrix, nInt), init);

            return res[0][0];
        }
    }
}