/*
 * Michael Persico
 * Oct. 08,2021
 * Simple beads count
 * https://www.codewars.com/kata/58712dfa5c538b6fc7000569/
*/

fun countRedBeads(nBlue: Int): Int = if (nBlue >= 2) nBlue + (nBlue - 2) else  0

fun main(args: Array<String>) {
    println(countRedBeads(0)) // 0
    println(countRedBeads(1)) // 0
    println(countRedBeads(3)) // 4
    println(countRedBeads(5)) // 8
}
