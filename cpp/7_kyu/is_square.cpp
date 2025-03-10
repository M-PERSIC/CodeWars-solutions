/*
* SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
*
* SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/54c27a33fb7da0db0100040e

#include <iostream>

bool is_square(int n)
{
  if (!(n < 0)) {
    for (int i = 0; i <= n; i++) {
      if (i * i == n) {
        return true;
      }
    }
  }
  return false;
}

int main() {
    std::cout << is_square(-1) << std::endl; // 0 (false)
    std::cout << is_square(0) << std::endl; // 1 (true)
    std::cout << is_square(3) << std::endl; // 0
    std::cout << is_square(4) << std::endl; // 1
    std::cout << is_square(25) << std::endl; // 1
    std::cout << is_square(26) << std::endl; // 0

}
