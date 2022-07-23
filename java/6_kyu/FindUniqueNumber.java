/*
Michael Persico
Oct.03, 2021
Find the unique number 
https://www.codewars.com/kata/585d7d5adb20cf33cb000235
*/

import java.util.stream.*;
import java.util.*;

public class FindUniqueNumber {
    public static double findUniq(double arr[]) {
        double min = DoubleStream.of(arr).min().getAsDouble();
        double max = DoubleStream.of(arr).max().getAsDouble();
        double sum = DoubleStream.of(arr).sum();
        return DoubleStream.of(arr).filter(i -> (sum - i)/(arr.length - 1) == max || (sum - i)/(arr.length - 1) == min).findFirst().getAsDouble();
    }
    public static void main(String[] args) {
        System.out.println(findUniq(new double[]{0,1,0})); // 1.0
        System.out.println(findUniq(new double[]{1,1,1,2,1,1})); // 2.0
    }
}
