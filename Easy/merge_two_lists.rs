// Leetcode provided the boilerplate for this one
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy;
        let mut l1 = list1;
        let mut l2 = list2;
        
        while let (Some(node1), Some(node2)) = (l1.as_ref(), l2.as_ref()) {
            if node1.val <= node2.val {
                current.next = l1;
                current = current.next.as_mut().unwrap();
                l1 = current.next.take();
            } else {
                current.next = l2;
                current = current.next.as_mut().unwrap();
                l2 = current.next.take();
            }
        }
        
        current.next = if l1.is_some() { l1 } else { l2 };
        dummy.next
    }
}
