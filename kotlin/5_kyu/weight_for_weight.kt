/*
 * Michael Persico
 * Oct.05, 2021
 * Weight for weight
 * https://www.codewars.com/kata/55c6126177c9441a570000cc
*/

fun orderWeight(string: String): String {
   
    var weights = string.split(" ").map { it.toString().map { it.toString().toInt() } }.map { it.sum() }
    var sortedWeightMap = string.split(" ").zip(weights).sortedWith(compareBy({ it.second }, { it.first }))
    var str: String = ""
    sortedWeightMap.forEach { str += it.first + " "}
    return str.trimEnd()
}

fun main(args: Array<String>) {
    println(orderWeight("56 65 74 100 99 68 86 180 90")) // "100 180 90 56 65 74 68 86 99"
    println(orderWeight("2000 10003 1234000 44444444 9999 11 11 22 123")) //"11 11 2000 10003 22 123 1234000 44444444 9999"
}
