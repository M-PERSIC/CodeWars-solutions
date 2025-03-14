/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/57eb8fcdf670e99d9b000272

fun high(str: String): String = str.split(" ").maxByOrNull { it.sumOf { it.toInt() - 96 } }!!

fun main(args: Array<String>) {
    println(high("what time are we climbing up the volcano")) // volcano
}
