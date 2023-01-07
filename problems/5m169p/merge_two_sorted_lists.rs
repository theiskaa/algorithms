// Merge Two Sorted Lists -  https://leetcode.com/problems/merge-two-sorted-lists

/* Problem Statement:
   You are given the heads of two sorted linked lists list1 and list2.

   Merge the two lists in a one sorted list. The list should be made by splicing together the 
   nodes of the first two lists.

   Return the head of the merged linked list.
*/

/* Examples:
   Input: list1 = [1,2,4], list2 = [1,3,4]
   Output: [1,1,2,3,4,4]

   Input: list1 = [], list2 = []
   Output: []

   Input: list1 = [], list2 = [0]
   Output: [0]
*/

/* Constraints:
  • The number of nodes in both lists is in the range [0, 50].
  • -100 <= Node.val <= 100
  • Both list1 and list2 are sorted in non-decreasing order.
*/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { val, next: None }
  }

  #[inline]
  fn next(val: i32, next: ListNode) -> Self {
    ListNode { val, next: Some(Box::new(next)) }
  }

  //
  // Part of alogirthm implementation.
  // Used for comparing linked lists.
  //
  fn convert_to_vector(&self) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::from([self.val.clone()]);

    let mut n: Option<Box<ListNode>> = self.next.clone();
    while n != None {
      res.push(n.clone().unwrap().val);
      n = n.clone().unwrap().next;
    }

    res
  }
}

fn main() {
    let list1 = Some(Box::new(ListNode::next(1, ListNode::next(2, ListNode::new(4)))));
    let list2 = Some(Box::new(ListNode::next(1, ListNode::next(3, ListNode::new(4)))));
    let expected: Vec<i32> = vec![1, 1, 2, 3, 4, 4];

    let res: Option<Box<ListNode>> = merge_two_lists(list1, list2);
    match res {
       None => println!("got: NONE"),
       Some(v) => {
         let got = v.convert_to_vector();
         if expected != got { 
              println!("error: expected: {:?} | got: {:?}", expected, got);
         } else {
              println!("Merge Two Sorted List: {:?}", got);
         }
       }
    }
}

// A recursion implementation, which uses insertion to arrange values.
pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  return match (list1, list2) {
    (None, None) => None,
    (Some(l), None) => Some(l),
    (None, Some(r)) => Some(r),
    (Some(l), Some(r)) => {
      if l.val <= r.val {
        Some(Box::new(ListNode{val: l.val, next: merge_two_lists(l.next, Some(r))}))
      } else {
        Some(Box::new(ListNode{val: r.val, next: merge_two_lists(r.next, Some(l))}))
      }
    }
  };
}

