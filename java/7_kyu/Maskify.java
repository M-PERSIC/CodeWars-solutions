/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/5412509bd436bd33920011bc

public class Maskify {

    public static String maskify(String str) {
            if (str.length() > 4) {
            String maskedNum = str.substring(0, str.length() -4).replaceAll(".", "#");
            return maskedNum + str.substring(str.length() -4);
            }
        return str;
        }
        
    public static void main(String[] args) {
        System.out.println(maskify("4556364607935616")); //############5616
        System.out.println(maskify("64607935616")); //#######5616
        System.out.println(maskify("1")); //1
        System.out.println(maskify("")); //""
        System.out.println(maskify("Skippy")); //##ippy
        System.out.println(maskify("Nananananananananananananananana Batman!")); //####################################man!

    }

}
