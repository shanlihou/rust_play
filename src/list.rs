#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode <T> {
    pub val: T,
    pub prev: Option<Box<ListNode<T>>>,
    pub next: Option<Box<ListNode<T>>>,
}

impl <T> ListNode<T> {
    #[inline]
    fn new(val: T) -> Self {
        ListNode {prev:None, next: None, val }
    }
}

pub struct List<T> {
    pub head: Option<Box<ListNode<T>>>,
    pub tail: Option<Box<ListNode<T>>>,
}

impl <T> List<T> {
    fn push_back(&self, val:T) {
        let mut node = ListNode::new(val);

    }
}

pub fn test() {
    let mut head = ListNode::new(1);
    head.next = Some(Box::new(ListNode::new(2)));
}