/*
 * Michael Persico
 * Oct. 08,2021
 * How many arguments
 * https://www.codewars.com/kata/5c44b0b200ce187106452139
*/

fun argsCount(vararg args: Any): Int = args.size

fun main(args: Array<String>) {
    println(argsCount(1,2,3,10)) // 4
}
