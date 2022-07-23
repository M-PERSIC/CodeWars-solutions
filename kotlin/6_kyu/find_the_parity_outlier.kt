/*
* Michael Persico
* Sept.28, 2021
* Find The Parity Outlier
* https://www.codewars.com/kata/5526fc09a1bbd946250002dc
*/

fun find(integers: Array<Int>): Int {
    val oddArr: List<Int> = integers.filter{it % 2 == 0}
    if (oddArr.size == 1)
        return oddArr[0]
    return integers.filter{it % 2 != 0}[0]
}

fun main(args: Array<String>) {
    println(find(arrayOf(2,6,8,-10,3))) // 3
    println(find(arrayOf(206847684,1056521,7,17,1901,21104421,7,1,35521,1,7781))) // 20687684
    println(find(arrayOf(Integer.MAX_VALUE, 0, 1))) // 0
}
