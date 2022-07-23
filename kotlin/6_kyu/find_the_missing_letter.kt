/* Michael Persico
* Sept. 28, 2021
* Find the missing letter
* https://www.codewars.com/kata/5839edaa6754d6fec10000a2
*/

fun findMissingLetter(array: CharArray): Char {
    val fullList: List<Int> = (0 .. array[array.size - 1].toInt() - array[0].toInt() step 1)
                              .toList()
                              .map { it + array[0].toInt() }
    return (fullList.minus(array.toList().map { it.toInt() }))[0].toChar()
}

fun main(args: Array<String>) {
    println(findMissingLetter(charArrayOf('a','b','c','d','f'))) // 'e'
    println(findMissingLetter(charArrayOf('O','Q','R','S'))) // 'P'
}
