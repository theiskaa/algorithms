// Balanced Binary Tree - https://leetcode.com/problems/balanced-binary-tree

/* Problem Statement:
   Given a binary tree, determine if it is Height-Balanced(A height-balanced
   binary tree is a binary tree in which the depth of the two subtrees of
   every node never differs by more than one.)
*/

/* Example:
   1.
    Input: root = [3,9,20,null,null,15,7]
    Output: true
    Image: https://assets.leetcode.com/uploads/2020/10/06/balance_1.jpg

   2.
    Input: root = [1,2,2,3,3,null,null,4,4]
    Output: false
    Image: https://assets.leetcode.com/uploads/2020/10/06/balance_2.jpg

   3.
    Input: root = []
    Output: true
*/

/* Solution:
   Main function Checks root's, root.Left's and root.Right's left height - righ height and returns final resut.
   height of TreeNode is generated by height function, which calls itself recursively for bassed root's left and right.
   -> height takes max of root's left side's height and right side's height + 1, this continues recursively until seeing nil node.
*/

package main

import (
	"fmt"
	"math"
)

func main() {
	tests := []struct {
		root     *TreeNode
		expected bool
	}{
		{
			expected: true,
			root: &TreeNode{
				Val:  3,
				Left: &TreeNode{Val: 9},
				Right: &TreeNode{
					Val:   20,
					Left:  &TreeNode{Val: 15},
					Right: &TreeNode{Val: 7},
				},
			},
		},
		{
			expected: false,
			root: &TreeNode{
				Val:   1,
				Right: &TreeNode{Val: 2},
				Left: &TreeNode{
					Val:   2,
					Right: &TreeNode{Val: 3},
					Left: &TreeNode{
						Val:   3,
						Left:  &TreeNode{Val: 4},
						Right: &TreeNode{Val: 4},
					},
				},
			},
		},
		{
			expected: true,
			root: &TreeNode{Val: 1,
				Right: &TreeNode{Val: 3, Left: &TreeNode{Val: 6}},
				Left: &TreeNode{
					Right: &TreeNode{Val: 5},
					Val:   2, Left: &TreeNode{Val: 4, Left: &TreeNode{Val: 8}},
				},
			},
		},
	}

	fmt.Println("Balanced Binary Tree:")
	i := 1
	for _, a := range tests {
		res := isBalanced(a.root)
		fmt.Printf(" %d'th test result: %v expected: %v \n", i, res, a.expected)
		i += 1
	}
}

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isBalanced(root *TreeNode) bool {
	if root == nil {
		return true
	}

	left := height(root.Left)
	right := height(root.Right)

	return math.Abs(float64(left-right)) <= 1 && isBalanced(root.Left) && isBalanced(root.Right)
}

func height(root *TreeNode) int {
	if root == nil {
		return 0
	}

	return int(math.Max(float64(height(root.Left)), float64(height(root.Right)))) + 1
}
