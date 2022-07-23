/* Michael Persico
 * Sept. 25, 2021
 * Find the odd int
 * https://www.codewars.com/kata/54da5a58ea159efa38000836
 */

#include <vector>
#include <set>
#include <algorithm>
#include <iostream>

int findOdd(const std::vector<int>& numbers){
  std::set<int> integers(numbers.begin(), numbers.end());
  int oddNum = 0;
  for (int num : integers) {
    if (std::count(numbers.begin(), numbers.end(), num) % 2 != 0) {
      oddNum = num;
    }
  }
  return oddNum;
}

int main() {
    std::cout << findOdd(std::vector<int>{20,1,-1,2,-2,3,3,5,5,1,2,4,20,4,-1,-2,5}) << std::endl; // 5
    std::cout << findOdd(std::vector<int>{1,1,2,-2,5,2,4,4,-1,-2,5}) << std::endl; // -1
    std::cout << findOdd(std::vector<int>{20,1,1,2,2,3,3,5,5,4,20,4,5}) << std::endl; // 5
    std::cout << findOdd(std::vector<int>{10}) << std::endl; // 10
    std::cout << findOdd(std::vector<int>{1,1,1,1,1,1,10,1,1,1,1}) << std::endl; // 10
}
