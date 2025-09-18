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
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut carry = 0;
        let mut a = 0;
        let mut b = 0;
        
        loop{
            let mut c = 0;
            if let Some(n1) = l1 {
                a =n1.val;
                l1 = n1.next;
                c += 1;
            }
            if let Some(n2) = l2{
                b = n2.val;
                l2 = n2.next;
                c += 1;
            }
            if c == 0 {break}
            let s = a + b + carry;
            let n = s % 10;
            carry = s / 10;
            
            tail.next = Some(Box::new(ListNode::new(n)));
            tail = tail.next.as_mut().unwrap();
            a = 0; b =0;
        }
        if carry == 1 {
            tail.next = Some(Box::new(ListNode::new(1)));
        }

        return dummy.next;
    }
}
