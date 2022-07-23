/*
* Michael Persico
* Oct.08, 2021
* Small enough? - Beginner
* https://www.codewars.com/kata/57cc981a58da9e302a000214
*/

fun smallEnough(a : IntArray, limit : Int) : Boolean = !a.any { it > limit }

fun main(args: Array<String>) {
   println(smallEnough(intArrayOf(66, 101), 200))) // true
}
