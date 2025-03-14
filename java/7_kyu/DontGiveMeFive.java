/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/5813d19765d81c592200001a

import java.util.stream.*;

public class DontGiveMeFive {

    public static int dontGiveMeFive(int start, int end) {
        return IntStream.range(start, end + 1).filter(i -> !Integer.toString(i).contains("5")).boxed().collect(Collectors.toList()).size();
    }
    
    public static void main(String[] args) {
        System.out.println(dontGiveMeFive(1,9)); // 8
        System.out.println(dontGiveMeFive(4,17)); // 12
    }

}
