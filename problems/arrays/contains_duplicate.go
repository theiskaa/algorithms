// Contains Duplicate - https://leetcode.com/problems/contains-duplicate/

/* Problem Statement:
 * Given an integer array nums, return true if any value appears at
 * least twice in the array, and return false if every element is distinct.
 */

/* Examples:
 *
 * Input: prices = [1,2,3,1]
 * Output: true
 * Explanation: In this case, `1` is duplicated.
 *
 * Input: prices = [1,2,3,4]
 * Output: true
 * Explanation: In this case, there is not duplications.
 */

package main

import "fmt"

func main() {
	tests := map[bool][]int{
		true:  {1, 2, 3, 1},
		false: {1, 2, 3, 4},
	}

	i := 1
	for expected, data := range tests {
		res := containsDuplicate(data)
		if res != expected {
			println(fmt.Sprintf("Error: Got wrong result: expected: %v, got: %v", expected, res))
		} else {
			println(fmt.Sprintf("Success: %v'st test passed", i))
		}

		i += 1
	}
}

func containsDuplicate(nums []int) bool {
	collection := make(map[int]int)

	for _, n := range nums {
		if _, ok := collection[n]; ok {
			return true
		} else {
			collection[n] = 1
		}
	}

	return false
}
