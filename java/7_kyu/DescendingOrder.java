/*
Michael Persico
Oct.09, 2021
Descending Order
https://www.codewars.com/kata/5467e4d82edf8bbf40000155
*/

import java.util.*;

public class DescendingOrder {

    public static int sortDesc(final int num) {
        String result = "";
        char[] digits = Integer.toString(num).toCharArray();
        Arrays.sort(digits);
        for (int i = digits.length - 1; i >= 0; i--) {
        result += digits[i];
        }
        return Integer.parseInt(result);
    }
    
    public static void main(String[] args) {
        System.out.println(sortDesc(123456789)); // 987654321
    }
}
