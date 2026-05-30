#[derive(Debug, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    pub fn from_vec(mut values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        while let Some(v) = values.pop() {
            let mut node = Box::new(Self::new(v));
            node.next = head;
            head = Some(node);
        }
        head
    }
}
