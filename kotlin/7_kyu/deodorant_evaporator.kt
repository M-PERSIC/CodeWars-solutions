/*
 * Michael Persico
 * Sept.28, 2021
 * Deodorant Evaporator
 * https://www.codewars.com/kata/5506b230a11c0aeab3000c1f
*/

fun evaporator(content: Double, evap_per_day: Double, threshold: Double) : Int {
    var content_level: Double = content
    var day: Int = 0
    var evap_limit: Double = content * (threshold / 100.0);
    while (content_level > evap_limit) {
        content_level *= (1.0 - (evap_per_day / 100.0))
        day++
    }
    return day
}

fun main(args: Array<String>) {
    println(evaporator(10.0,10.0,10.0)) // 22
    println(evaporator(10.0,10.0,5.0)) // 29
    println(evaporator(100.0,5.0,5.0)) // 59
}
