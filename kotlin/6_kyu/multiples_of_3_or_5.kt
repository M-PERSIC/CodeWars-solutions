/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/514b92a657cdc65150000006

fun solution(number: Int): Int = (0 until number).toList().filter { it % 3 == 0 || it % 5 == 0 }.sum()

fun main(args: Array<String>) {
   println(solution(10)) // 23
   println(solution(200)) // 9168
}
