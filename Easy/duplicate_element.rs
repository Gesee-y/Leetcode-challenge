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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if !head.is_some(){ return head}
        let mut dummy = Box::new(ListNode::new(-854));
        let mut tail = &mut dummy;
        let mut current :Box<ListNode>= head.expect("failed.");
        loop{
            if current.val != tail.val {
                tail.next = Some(Box::new(ListNode::new(current.val)));
                tail = tail.next.as_mut().expect("Failed.");
                
            }
            current = if current.next.is_some() {current.next.expect("nothing")} else {break}
            
        }
        return dummy.next;
    }
}
