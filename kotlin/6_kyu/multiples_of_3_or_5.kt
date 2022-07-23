/*
* Michael Persico
* Oct.08, 2021
* Multiples of 3 or 5
* https://www.codewars.com/kata/514b92a657cdc65150000006
*/

fun solution(number: Int): Int = (0 until number).toList().filter { it % 3 == 0 || it % 5 == 0 }.sum()

fun main(args: Array<String>) {
   println(solution(10)) // 23
   println(solution(200)) // 9168
}
