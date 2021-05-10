use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<node::Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.fold_while(0, |accum, _| (accum + 1, true))
    }

    pub fn push(&mut self, _element: T) {
        match self.head.take() {
            None => self.head = Some(Box::new(node::Node::new(_element))),
            Some(n) => self.head = Some(Box::new(node::Node::new(_element).with_next(n))),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(bn) => {
                let (new_head, val) = bn.decompose();
                self.head = new_head;
                Some(val)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(bn) => Some(bn.value()),
            None => None,
        }
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        if self.is_empty() {
            self
        } else {
            let mut new_list = Self::new();
            while let Some(n) = self.pop() {
                new_list.push(n);
            }
            new_list
        }
    }

    fn fold_while<A: Copy>(
        &self,
        mut accum: A,
        fold_accum_func: fn(accum: A, node: &Box<node::Node<T>>) -> (A, bool),
    ) -> A {
        let mut node = &self.head;
        while let Some(boxed_node) = node {
            let (acc, continue_folding) = fold_accum_func(accum, boxed_node);
            accum = acc;
            if !continue_folding {
                break;
            }
            node = boxed_node.next();
        }
        accum
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut s = Self::new();
        for t in iter {
            s.push(t);
        }
        s
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut v = Vec::new();
        while let Some(t) = self.pop() {
            v.push(t);
        }
        v.reverse();
        v
    }
}

mod node {
    pub struct Node<T> {
        value: T,
        next: Option<Box<Node<T>>>,
    }

    impl<T> Node<T> {
        pub fn new(element: T) -> Self {
            Self {
                value: element,
                next: None,
            }
        }

        pub fn with_next(mut self, node: Box<Self>) -> Self {
            self.next = Some(node);
            self
        }

        pub fn next(&self) -> &Option<Box<Node<T>>> {
            &self.next
        }

        pub fn value(&self) -> &T {
            &self.value
        }

        pub fn decompose(self) -> (Option<Box<Node<T>>>, T) {
            (self.next, self.value)
        }
    }
}
