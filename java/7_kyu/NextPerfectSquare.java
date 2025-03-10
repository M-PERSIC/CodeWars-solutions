/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/56269eb78ad2e4ced1000013

import java.lang.Math;

public class NextPerfectSquare {

    public static long findNextSquare(long sq) {
        return (Math.sqrt(sq) % 1 == 0.0) ? (long)Math.pow(Math.sqrt(sq) + 1, 2) : -1;
    }
    
    public static void main(String[] args) {
        System.out.println(findNextSquare(625)); // 676
    }
}
