// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            // If both are empty, return None
            (None, None) => None,
            
            // If one is empty, return the other
            (Some(n), None) | (None, Some(n)) => Some(n),
            
            // If both have nodes, compare their values, and take smaller
            (Some(mut l1), Some(mut l2)) => {
                if l1.val < l2.val {
                    l1.next = Solution::merge_two_lists(l1.next, Some(l2));
                    Some(l1)
                } else {
                    l2.next = Solution::merge_two_lists(Some(l1), l2.next);
                    Some(l2)
                }
            }
        }
    }
}