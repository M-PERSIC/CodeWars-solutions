// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/59473c0a952ac9b463000064

package main

import "fmt"

func AlphabetWar(fight string) string {
	var (
		left_power   = map[rune]int{'t': 0, 's': 1, 'b': 2, 'p': 3, 'w': 4}
		left_priest  = map[int]rune{0: 't', 1: 's', 2: 'b', 3: 'p', 4: 'w'}
		right_power  = map[rune]int{'j': 0, 'z': 1, 'd': 2, 'q': 3, 'm': 4}
		right_priest = map[int]rune{0: 'j', 1: 'z', 2: 'd', 3: 'q', 4: 'm'}
		transformed  = []rune(fight)
		counter      = 0
	)
	IsInRange := func(index int) bool {
		return index >= 0 && index < len(transformed)
	}
	Convert := func(letter rune, index int) {
		if letter != 'j' && letter != 't' {
			switch {
			case (IsInRange(index-1) && !IsInRange(index+1) && transformed[index-1] == 't') ||
				(IsInRange(index+1) && !IsInRange(index-1) && transformed[index+1] == 't') ||
				(IsInRange(index-1) && IsInRange(index+1) && transformed[index-1] == 't' && transformed[index+1] != 'j') ||
				(IsInRange(index-1) && IsInRange(index+1) && transformed[index+1] == 't' && transformed[index-1] != 'j'):
				if power, ok := right_power[letter]; ok {
					transformed[index] = left_priest[power]
				}
			case (IsInRange(index-1) && !IsInRange(index+1) && transformed[index-1] == 'j') ||
				(IsInRange(index+1) && !IsInRange(index-1) && transformed[index+1] == 'j') ||
				(IsInRange(index-1) && IsInRange(index+1) && transformed[index-1] == 'j' && transformed[index+1] != 't') ||
				(IsInRange(index-1) && IsInRange(index+1) && transformed[index+1] == 'j' && transformed[index-1] != 't'):
				if power, ok := left_power[letter]; ok {
					transformed[index] = right_priest[power]
				}
			default:
			}
		}
	}
	for index, letter := range transformed {
		Convert(letter, index)
	}
	for _, letter := range transformed {
		if power, ok := left_power[letter]; ok {
			counter += power
		} else {
			counter -= right_power[letter]
		}
	}
	switch {
	case counter > 0:
		return "Left side wins!"
	case counter < 0:
		return "Right side wins!"
	default:
		return "Let's fight again!"
	}
}

func main() {
	fmt.Println(AlphabetWar("zt"))         // Left side wins!
	fmt.Println(AlphabetWar("sj"))         // Right side wins!
	fmt.Println(AlphabetWar("azt"))        // Left side wins!
	fmt.Println(AlphabetWar("tzj"))        // Right side wins!
	fmt.Println(AlphabetWar("wololooooo")) // Left side wins!
	fmt.Println(AlphabetWar("zdqmwpbs"))   // Let's fight again!
	fmt.Println(AlphabetWar("ztztztzs"))   // Left side wins!
	fmt.Println(AlphabetWar("jjjjpt"))     // Left side wins!
}
