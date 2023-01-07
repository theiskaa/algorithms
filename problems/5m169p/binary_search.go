// Binary Search - https://leetcode.com/problems/binary-search

/* Problem Statement:
   Given an array of integers nums which is sorted in ascending order,
   and an integer target, write a function to search target in nums.
   If target exists, then return its index. Otherwise, return -1.

   You must write an algorithm with O(log n) runtime complexity.
*/

/* Examples:
   Input: nums = [-1,0,3,5,9,12] & target = 9
   Output: 4
   Explanation: 9 exists in nums and its index is 4
*/

package main

import (
	"fmt"
	"math"
)

func main() {
	tests := map[int][]int{
		9: {-1, 0, 3, 5, 9, 12},
		5: {-12, -10, -1, 0, 3, 5, 9, 12},
                99: {-12, -10, -1, 0, 3, 5, 9, 12},
	}

	i := 1
	for target, nums := range tests {
		res := search(nums, target)
                fmt.Printf("%d test | target: %d found at: %d\n", i, target, res)
		i += 1
	}
}

func search(nums []int, target int) int {
    low, high := 0, len(nums)

    for low < high {
        middle := int(math.Floor(float64((low + high) / 2)))
        current := nums[middle]

        if current == target {
            return middle
        } else if current > target {
            high = middle
        } else {
            low = middle + 1
        }
    }

    return -1 
}
