/*
Michael Persico
Oct.15, 2021
Counting Duplicates
https://www.codewars.com/kata/54bf1c2cd5b56cc47f0007a1
*/

import java.util.HashMap;
import java.util.*;
import java.util.stream.*;

public class CountingDuplicates {

    public static int duplicateCount(String text) {
        Map<String, Integer> letters = new HashMap<String, Integer>();
        Arrays.stream(text.toLowerCase().split("")).forEach(letter -> letters.merge(letter, 1, Integer::sum));
        return (int)(letters.keySet().stream().filter(letter -> letters.get(letter) > 1).count());
    }
    
    public static void main(String[] args) {
        System.out.println(duplicateCount("abcdea")); // 1
    }
}
