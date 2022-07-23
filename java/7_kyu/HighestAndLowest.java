/*
Michael Persico
Oct.01, 2021
Highest and Lowest 
https://www.codewars.com/kata/554b4ac871d6813a03000035
*/

import java.util.stream.*;
import java.util.*;

public class HighestAndLowest {

    public static String HighAndLow(String numbers) {
        List num = Arrays.stream(numbers.split(" ")).map(i -> Integer.parseInt(i)).collect(Collectors.toList());
        return String.format("%d %d", Collections.max(num), Collections.min(num));
    }
    
    public static void main(String[] args) {
        System.out.println(HighAndLow("1 2 3 4 5")); // 5 1
        System.out.println(HighAndLow("8 3 -5 42 -1 0 0 -9 4 7 4 -4")); // 42 -9
    }
}
