/*
 * Michael Persico
 * Oct. 08,2021
 * Printer Errors
 * https://www.codewars.com/kata/56541980fa08ab47a0000040
*/

fun printerError(s: String): String = String.format("%d/%d", s.length - "[a-m]".toRegex().findAll(s).count(), s.length)

fun main(args: Array<String>) {
    print(printerError("aaabbbbhaijjjm")) // 0/14
    print(printerError("aaaxbbbbyyhwawiwjjjwwm")) // 8/22
    
}
