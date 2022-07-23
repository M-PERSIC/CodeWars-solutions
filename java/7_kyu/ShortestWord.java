/* Michael Persico
 * Sept 26, 2021
 * Shortest Word
 * https://www.codewars.com/kata/57cebe1dc6fdc20c57000ac9
 */

import java.util.Arrays;
import java.util.List;
import java.util.stream.*;

public class ShortestWord {

    public static int FindShort(String s) {
        List<Integer> lengths = Arrays.asList(s.split(" "))
                                        .stream()
                                        .map(x -> x.length())
                                        .sorted()
                                        .collect(Collectors.toList());
        return lengths.get(0);
    }
    
    public static void main(String[] args) {
        System.out.println(FindShort("bitcoin take over the world maybe who knows perhaps")); // 3
	System.out.println(FindShort("turns out random test cases are easier than writing out basic ones")); //3
	System.out.println(FindShort("Let's travel abroad shall we")); // 2
    }
}
