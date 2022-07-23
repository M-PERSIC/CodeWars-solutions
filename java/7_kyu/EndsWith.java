/*
Michael Persico
Sept.30, 2021
String ends with?
https://www.codewars.com/kata/51f2d1cafc9c0f745c0037d
*/

public class EndsWith {

    public static boolean solution(String str, String ending) {
        return str.substring(str.length() < ending.length() ? 0 : str.length() - ending.length()).equals(ending);
    }
    
    public static void main(String[] args) {
        System.out.println(solution("abc", "bc")); // true
        System.out.println(solution("abc", "d")); // false
        System.out.println(solution("samurai", "ai")); // true
        System.out.println(solution("samurai", "ra")); // false
        System.out.println(solution("a", "bb")); // false


    }

}
