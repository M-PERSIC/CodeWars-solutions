/*
 * Michael Persico
 * Sept. 28, 2021
 * Encrypt this!
 * https://www.codewars.com/kata/5848565e273af816fb000449
*/

fun encryptThis(text:String): String = text.split(" ").map { it.first().toInt().toString() + (if (it.length > 1) it.last() else "") + (if (it.length > 2) it.substring(2, it.length - 1) + it.get(1) else "") }.joinToString(" ")

fun main(args: Array<String>) {
    println(encryptThis("A wise old owl lived in an oak")) // 65 119esi 111dl 111lw 108dvei 105n 97n 111ka
}
