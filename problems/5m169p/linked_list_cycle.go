// Linked List Cycle - https://leetcode.com/problems/linked-list-cycle

/* Problem Statement:
   Given head, the head of a linked list, determine if the linked list has a cycle in it.

   There is a cycle in a linked list if there is some node in the list that can be
   reached again by continuously following the next pointer. Internally, pos is
   used to denote the index of the node that tail's next pointer is connected to.
   Note that pos is not passed as a parameter.

   Return true if there is a cycle in the linked list. Otherwise, return false.
*/

/* Examples:
   1.
    Input: head = [3,2,0,-4], pos = 1
    Output: true
    Explanation: There is a cycle in the linked list, where the tail
                 connects to the 1st node (0-indexed).
    Image: https://assets.leetcode.com/uploads/2018/12/07/circularlinkedlist.png

   2.
     Input: head = [1,2], pos = 0
     Output: true
     Explanation: There is a cycle in the linked list, where the tail connects to the 0th node.
     Image: https://assets.leetcode.com/uploads/2018/12/07/circularlinkedlist_test2.png

   3.
     Input: head = [1], pos = -1
     Output: false
     Explanation: There is no cycle in the linked list.
     Image: https://assets.leetcode.com/uploads/2018/12/07/circularlinkedlist_test3.png
*/

/* Solution:
   Crates a HashSet alike collection, where only keys matter.
   Adds each current.Next ListNode to collection as element of HashSet,
   and checks if that values already placed in there or not.
   If yes, returns true. if not goes to outside of loop and returns false.
*/

package main

import (
	"fmt"
)

func main() {
	tests := []struct {
		root     *ListNode
		expected bool
	}{
		{},
	}

	fmt.Println("Linked List Cycle:")
	i := 1
	for _, a := range tests {
		res := hasCycle(a.root)
		fmt.Printf(" %d'th test result: %v expected: %v \n", i, res, a.expected)
		i += 1
	}
}

type ListNode struct {
	Val  int
	Next *ListNode
}

func hasCycle(head *ListNode) bool {
	collection := map[ListNode]int{}

	current := head
	for current != nil {
		if _, has := collection[*current]; has {
			return true
		}

		collection[*current] = 1
		current = current.Next
	}

	return false
}
