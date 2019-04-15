import java.io.InputStreamReader;
import java.io.BufferedReader;
import java.lang.StringBuilder;
import java.math.BigInteger;
import java.util.*;

public class GCJCryptopangramJava {
    static BigInteger gcd(BigInteger n, BigInteger m) {
        if (m.equals(BigInteger.ZERO)) {
            return n;
        }
        return gcd(m, n.mod(m));
    }
    public static void main(String[] args) {
        Scanner input = new Scanner(new BufferedReader(new InputStreamReader(System.in)));
        Integer t = input.nextInt();
        for (int i=0; i<t; i++) {
            BigInteger n = new BigInteger(input.next());
            int l = input.nextInt();
            BigInteger[] ciphers = new BigInteger[l];
            for (int j = 0; j<l; j++) {
                ciphers[j] = new BigInteger(input.next());
            }
            LinkedList<BigInteger> primes = new LinkedList<BigInteger>();
            BigInteger comm = gcd(ciphers[0], ciphers[1]);
            BigInteger prevPrime = ciphers[0].divide(comm);
            primes.add(prevPrime);
            for (int j=0; j<l-1; j++) {
                BigInteger newPrime = ciphers[j].divide(prevPrime);
                primes.add(newPrime);
                prevPrime = newPrime;
            }
            primes.add(ciphers[l-1].divide(prevPrime));

            Set<BigInteger> uniquePrimes = new HashSet<>();
            for (BigInteger p:primes) {
                uniquePrimes.add(p);
            }
            BigInteger[] sortedPrimes = uniquePrimes.toArray(new BigInteger[0]);
            Arrays.sort(sortedPrimes);

            HashMap<BigInteger, Character> map = new HashMap<>();
            for (int j=0; j<sortedPrimes.length; j++) {
                map.put(sortedPrimes[j],(char)('A' + j));
            }
            StringBuilder res = new StringBuilder();
            for (int j=0; j<primes.size(); j++) {
                res.append(map.get(primes.get(j)));
            }
            System.out.println("Case #" + i + ": " + res);
        }
    }
}
