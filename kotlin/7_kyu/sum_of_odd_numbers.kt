/*
 * Michael Persico
 * Nov.03, 2021
 * Sum of odd numbers
 * https://www.codewars.com/kata/55fd2d567d94ac3bc9000064
*/

fun rowSumOddNumbers(n: Int): Int {
    if (n != 1) {
        val firstPosition: Int = (1..n - 1).reduce { sum, num -> sum + num }
        return IntRange(firstPosition, firstPosition + (n - 1)).map { (2 * it) + 1 }.sum()
    }
    return 1
}

fun main(args: Array<String>) {
    println(rowSumOddNumbers(1)) // 1
    println(rowSumOddNumbers(13)) // 2197
}
