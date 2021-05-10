use std::ptr::null_mut;

// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

struct LinkedListNode<T> {
    value: T,
    prev: *mut LinkedListNode<T>,
    next: *mut LinkedListNode<T>,
}

impl<T> LinkedListNode<T> {
    pub fn from(element: T) -> Self {
        Self {
            value: element,
            prev: std::ptr::null_mut(),
            next: std::ptr::null_mut(),
        }
    }
    fn with_prev(mut self, node: *mut LinkedListNode<T>) -> Self {
        self.prev = node;
        self
    }
    fn with_next(mut self, node: *mut LinkedListNode<T>) -> Self {
        self.next = node;
        self
    }
    fn next(&self) -> *mut LinkedListNode<T> {
        self.next
    }
    fn prev(&self) -> *mut LinkedListNode<T> {
        self.prev
    }
}

pub struct LinkedList<T> {
    head: *mut LinkedListNode<T>,
    tail: *mut LinkedListNode<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: null_mut(),
            tail: null_mut(),
            len: 0,
        }
    }

    fn assert_invariants(&self) {
        debug_assert!(
            ((self.head.is_null() & self.tail.is_null()) && self.len == 0)
                || (!self.head.is_null() && !self.tail.is_null() && self.len >= 1)
        )
    }

    pub fn is_empty(&self) -> bool {
        self.assert_invariants();
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.assert_invariants();
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        self.assert_invariants();
        let at = self.head;
        Cursor { list: self, at }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        self.assert_invariants();
        let at = self.tail;
        Cursor { list: self, at }
    }

    fn insert_before(
        &mut self,
        node: *mut LinkedListNode<T>,
        element: T,
    ) -> *mut LinkedListNode<T> {
        match node.is_null() {
            true => self.insert_first(element),
            false => {
                self.assert_invariants();
                let prev = unsafe { (*node).prev };
                let new_node = Box::into_raw(Box::new(
                    LinkedListNode::from(element)
                        .with_next(node)
                        .with_prev(prev),
                ));
                if !prev.is_null() {
                    unsafe { (*prev).next = new_node };
                }
                if node == self.head {
                    self.head = new_node;
                }
                unsafe {
                    (*node).prev = new_node;
                };
                self.len += 1;
                self.assert_invariants();
                new_node
            }
        }
    }

    fn insert_after(&mut self, node: *mut LinkedListNode<T>, element: T) -> *mut LinkedListNode<T> {
        match node.is_null() {
            true => self.insert_first(element),
            false => {
                self.assert_invariants();
                let next = unsafe { (*node).next };
                let new_node = Box::into_raw(Box::new(
                    LinkedListNode::from(element)
                        .with_prev(node)
                        .with_next(next),
                ));
                if !next.is_null() {
                    unsafe { (*next).prev = new_node };
                }
                if node == self.tail {
                    self.tail = new_node;
                }
                unsafe {
                    (*node).next = new_node;
                };
                self.len += 1;
                self.assert_invariants();
                new_node
            }
        }
    }

    fn remove_at(&mut self, node: &mut *mut LinkedListNode<T>) -> Option<T> {
        if node.is_null() {
            return None;
        }
        self.assert_invariants();
        let removing = node;
        let prev = unsafe { (**removing).prev() };
        let next = unsafe { (**removing).next() };
        match next.is_null() {
            true => self.tail = prev,
            false => match prev.is_null() {
                true => unsafe { (*next).prev = null_mut() },
                false => unsafe { (*prev).next = next },
            },
        }
        match prev.is_null() {
            true => self.head = next,
            false => match next.is_null() {
                true => unsafe { (*prev).next = null_mut() },
                false => unsafe { (*next).prev = prev },
            },
        }
        self.len -= 1;
        let result = unsafe { Box::from_raw(*removing) }.value;
        *removing = if next.is_null() { prev } else { next };
        self.assert_invariants();
        Some(result)
    }

    // NOTE: if called when self.head or self.tail is not null already this will leak memory
    // PANICS: if called on non-empty list
    fn insert_first(&mut self, element: T) -> *mut LinkedListNode<T> {
        assert!(self.head.is_null() && self.tail.is_null() && self.len == 0);
        let node = Box::into_raw(Box::new(LinkedListNode::from(element)));
        self.head = node;
        self.tail = node;
        self.len += 1;
        self.assert_invariants();
        node
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            list: self,
            at: self.head,
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cursor = self.cursor_front();
        while cursor.take().is_some() {}
    }
}

unsafe impl<T> Send for LinkedList<T> where T: Send {}
unsafe impl<T> Sync for LinkedList<T> {}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    at: *mut LinkedListNode<T>,
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        match self.at.is_null() {
            true => None,
            false => unsafe { Some(&mut (*self.at).value) },
        }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        match self.at.is_null() {
            true => None,
            false => {
                self.at = unsafe { (*self.at).next() };
                self.peek_mut()
            }
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        match self.at.is_null() {
            true => None,
            false => {
                self.at = unsafe { (*self.at).prev() };
                self.peek_mut()
            }
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        match self.at.is_null() {
            true => None,
            false => self.list.remove_at(&mut self.at),
        }
    }

    pub fn insert_before(&mut self, element: T) {
        let result = self.list.insert_before(self.at, element);
        if self.at.is_null() {
            self.at = result;
        }
    }

    pub fn insert_after(&mut self, element: T) {
        let result = self.list.insert_after(self.at, element);
        if self.at.is_null() {
            self.at = result;
        }
    }
}

pub struct Iter<'a, T> {
    #[allow(dead_code)]
    list: &'a LinkedList<T>,
    at: *const LinkedListNode<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    /// SAFETY: LinkedListNode.at, if non-null, will always point to a valid node
    ///         LinkedListNode.next() will always return null or a valid pointer to a valid node
    fn next(&mut self) -> Option<&'a T> {
        match self.at.is_null() {
            true => None,
            false => {
                let result = self.at;
                self.at = unsafe { (*self.at).next() };
                match self.at.is_null() {
                    true => None,
                    false => unsafe { Some(&(*result).value) },
                }
            }
        }
    }
}
