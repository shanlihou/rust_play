#[derive(Debug)]
struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

#[derive(Clone, Debug)]
struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}

impl<T> StackNode<T> {
    fn new(val: T) -> StackNode<T> {
        StackNode{val, next:None}
    }
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack{top: None}
    }

    fn push(&mut self, val: T) {
        let mut node = StackNode::<T>::new(val);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        let val = self.top.take();
        match val {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.val)
            }
        }
    }
}

pub fn test() {
    let mut a = Stack::<i32>::new();
    a.push(78);
    println!("a is {:?}", a);
}