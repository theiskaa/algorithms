// Lowest Common Ancestor of a Binary Search Tree - https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/

/* Problem Statement:
   Given a binary search tree (BST), find the lowest common ancestor (LCA) node of two given nodes in the BST.
   According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined between two nodes
   p and q as the lowest node in T that has both p and q as descendants
   (where we allow a node to be a descendant of itself).
*/

/* Examples:
   1.
    Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
    Output: 6
    Explanation: The LCA of nodes 2 and 8 is 6.

   2.
    Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
    Output: 2
    Explanation: The LCA of nodes 2 and 4 is 2, since a node can be a descendant of itself according to the LCA definition.

   3.
    Input: root = [2,1], p = 2, q = 1
    Output: 2
*/

/* Solution:
   Compare the q.Val and p.Val with current value.
   If both q and p are greater than current, replace current.Right to current.
   Or if both q and p are less than current, replace current.Left to current.
   - As result return current if none of the cases are applied.
*/

package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func main() {
	tests := []struct {
		root, p, q, expected *TreeNode
	}{
		{
			root: &TreeNode{
				Val: 6,
				Left: &TreeNode{
					Val:  2,
					Left: &TreeNode{Val: 0},
					Right: &TreeNode{
						Val:   4,
						Left:  &TreeNode{Val: 3},
						Right: &TreeNode{Val: 5},
					},
				},
				Right: &TreeNode{
					Val:   8,
					Left:  &TreeNode{Val: 7},
					Right: &TreeNode{Val: 9},
				},
			},
			p:        &TreeNode{Val: 2},
			q:        &TreeNode{Val: 8},
			expected: &TreeNode{Val: 6},
		},
	}

	fmt.Println("Lowest Common Ancestor:")
	i := 1
	for _, a := range tests {
		res := lowestCommonAncestor(a.root, a.p, a.q)
		fmt.Printf(" %d'th test result: %d expected: %d \n", i, res.Val, a.expected.Val)
		i += 1
	}
}

func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	current := root

	for true {
		if p.Val > current.Val && q.Val > current.Val {
			current = current.Right
			continue
		} else if p.Val < current.Val && q.Val < current.Val {
			current = current.Left
			continue
		}

		return current
	}

	return nil
}
