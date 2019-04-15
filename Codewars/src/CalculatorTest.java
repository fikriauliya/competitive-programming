import org.junit.Test;

import java.math.BigDecimal;
import java.util.Stack;

import static org.junit.Assert.assertEquals;

public class CalculatorTest {
    @Test
    public void simpleLiteral() {
        assertEquals("simple literal", new Double(127), Calculator.evaluate("127"));
    }

    @Test
    public void subtractionAndAddition() {
        assertEquals("addition", new Double(5), Calculator.evaluate("2 + 3"));
        assertEquals("subtraction", new Double(-5), Calculator.evaluate("2 - 3 - 4"));
    }

    @Test
    public void divisionAndMultiplication() {
        assertEquals("mixed division and multiplication", new Double(10), Calculator.evaluate("10 * 5 / 5"));
    }

    @Test
    public void allMixed() {
        assertEquals("mixed", new Double(13), Calculator.evaluate("2 / 2 + 3 * 4"));
    }

    @Test
    public void floats() {
        assertEquals("floats 1", new Double(0), Calculator.evaluate("7.7 - 3.3 - 4.4"));
    }
}

class Calculator {
    public static void calc(Stack<BigDecimal> numbers, Stack<String> operators) {
        String operator = operators.pop();
        BigDecimal number1 = numbers.pop();
        BigDecimal number2 = numbers.pop();
        if (operator.equals("*")) numbers.push(number1.multiply(number2));
        else if (operator.equals("/")) numbers.push(number2.divide(number1));
        else if (operator.equals("+")) numbers.push(number1.add(number2));
        else if (operator.equals("-")) numbers.push(number2.subtract(number1));
    }

    public static Double evaluate(String expression) {
        String[] tokens = expression.split(" ");
        Stack<BigDecimal> numbers = new Stack<>();
        Stack<String> operators = new Stack<>();
        for (int i=0; i<tokens.length; i++) {
            String token = tokens[i];
            if (token.equals("+") || token.equals("-") || token.equals("/") || token.equals("*")) {
                if (operators.size() >= 1 && (token.equals("+") || token.equals("-"))) calc(numbers, operators);
                operators.push(token);
            } else {
                numbers.push(new BigDecimal(token));
                if (operators.size() >= 1 && (operators.peek().equals("*") || operators.peek().equals("/")))
                    calc(numbers, operators);
            }
        }
        while (!operators.isEmpty()) calc(numbers, operators);

        return Double.parseDouble(numbers.pop().toString());
    }
}