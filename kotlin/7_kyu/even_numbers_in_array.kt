/*
 * Michael Persico
 * Oct. 08,2021
 * Even numbers in an array
 * https://www.codewars.com/kata/5a431c0de1ce0ec33a00000c
*/

fun evenNumbers(array: List<Int>, number: Int): List<Int> = array.filter{ it % 2 == 0}.takeLast(number)

fun main(args: Array<String>) {
    println(evenNumbers(listOf(1, 2, 3, 4, 5, 6, 7, 8, 9))) // 3
}
