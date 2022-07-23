/*
 * Michael Persico
 * Sept. 28, 2021
 * Which are in?
 * https://www.codewars.com/kata/550554fd08b86f84fe000a58
*/

fun inArray(array1: Array<String>, array2: Array<String>): Array<String> = array1.filter { str -> array2.any { it.contains(str) } }.distinct().sorted().toTypedArray()

fun main(args: Array<String>) {
    val a2: Array<String> = arrayOf<String>("lively", "alive", "harp", "sharp", "armstrong")
    val r1: Array<String> = inArray(arrayOf<String>("xyz", "live", "strong"), a2)
    r1.forEach { println(it) } // "live" "strong"
    val r2: Array<String> = inArray(arrayOf<String>("live", "strong", "arp"), a2)
    r2.forEach { println(it) } // "arp" "live" "strong"
    val r3: Array<String> = inArray(arrayOf<String>("tarp", "mice", "bull"), a2)
    r3.forEach { println(it) } // ""
}
