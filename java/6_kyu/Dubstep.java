/*
Michael Persico
Oct.01, 2021
Dubstep
https://www.codewars.com/kata/551dc350bf4e526099000ae5
*/

import java.util.*;
import java.util.stream.*;

public class Dubstep {

    public static String SongDecoder(String song) {
        return Arrays.asList(song.split("WUB")).stream().filter(i -> i.length() != 0).collect(Collectors.joining(" "));
    }
    
    public static void main(String[] args) {
        System.out.println(SongDecoder("WUBWUBABCWUB"));
        System.out.println(SongDecoder("RWUBWUBWUBLWUB"));
    }
}
