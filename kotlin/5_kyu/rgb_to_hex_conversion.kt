/*
 * Michael Persico
 * Oct.16, 2021
 * RGB To Hex Conversion
 * https://www.codewars.com/kata/513e08acc600c94f01000001/train/kotlin
*/

fun rgb(r: Int, g: Int, b: Int): String {
    var r_valid: Int = if (r > 255) 255 else if (r < 0) 0 else r
    var g_valid: Int = if (g > 255) 255 else if (g < 0) 0 else g
    var b_valid: Int = if (b > 255) 255 else if (b < 0) 0 else b
    return "%02X%02X%02X".format(r_valid, g_valid, b_valid)

}

fun main(args: Array<String>) {
    println(rgb(0,0,0)) // 000000
    println(rgb(0,0,-20)) // 000000
    println(rgb(300,255,255)) // FFFFFF
}
