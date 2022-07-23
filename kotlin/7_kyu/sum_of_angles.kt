/*
 * Michael Persico
 * Oct. 08,2021
 * Sum of angles
 * https://www.codewars.com/kata/5a03b3f6a1c9040084001765/
*/

fun angle(n: Int): Int = (n - 2) * 180

fun main(args: Array<String>) {
    println(angle(3)) // 180
    println(angle(4)) // 360
}
