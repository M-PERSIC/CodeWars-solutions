/*
 * Michael Persico
 * Oct. 08,2021
 * Vowel Count
 * https://www.codewars.com/kata/54ff3102c1bad923760001f3
*/

fun getCount(str : String) : Int = "[aeiouAEIOU]".toRegex().findAll(str).count()

fun main(args: Array<String>) {
    println(getCount("abracadabra")) // 5
    println(getCount("test")) // 1
    println(getCount("example")) // 3

}
