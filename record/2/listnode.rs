pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
  pub fn push(&mut self,val:i32){
    let newnode=ListNode::new(val);
    self.next=Some(Box::new(newnode));
  }

}