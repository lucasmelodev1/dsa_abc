use std::ptr;

/// Linked list which nodes only point to their next element.
/// 
/// Useful in cases where your data will grow indefinitely and your program 
/// can't handle Vec's memory copy in growth.
/// 
/// This implementation utilizes unsafe rust in some places due to the
/// complexity and runtime overhead of building a compile-time safe structure.
/// 
/// ### Examples
/// Here are some examples on how to use this structure
/// 
/// #### Inserting an element
/// 
/// ```
/// use dsa_abc::singly_linked_list::SinglyLinkedList;
/// 
/// // Creates the list with a starting element 10
/// let mut list = SinglyLinkedList::new(10);
/// assert_eq!(list.get_first(), Some(&10));
/// 
/// // Insert element at the end of list
/// list.push(20);
/// assert_eq!(list.get_last(), Some(&20));
/// 
/// // Insert element at the start of list
/// list.insert(1);
/// assert_eq!(list.get_first(), Some(&1));
/// ```
/// 
/// #### Deleting an element
/// 
/// ```
/// use dsa_abc::singly_linked_list::SinglyLinkedList;
/// 
/// // Creates an empty list
/// let mut list = SinglyLinkedList::new_empty();
/// list.push(10);
/// list.push(20);
/// list.push(30);
/// 
/// // Removes the first element
/// list.remove_first();
/// assert_eq!(list.get_first(), Some(&20)); // It was 10 before
/// 
/// // Removes the last element
/// list.pop();
/// assert_eq!(list.get_last(), Some(&20)); // It was 30 before
/// 
/// // Removes the element at the given index if it exists
/// list.remove_at(0);
/// assert_eq!(list.size, 0);
/// ```
/// 
pub struct SinglyLinkedList<T> {
    root: *mut Node<T>,
    leaf: *mut Node<T>,
    pub size: u32,
}

impl<T: PartialEq> SinglyLinkedList<T> {
    /// Creates a new singly linked list with specified data
    pub fn new(data: T) -> SinglyLinkedList<T> {
        let root = Node::new_mut(data);
        SinglyLinkedList {
            root,
            leaf: root,
            size: 1,
        }
    }

    /// Creates a new empty singly linked list
    pub fn new_empty() -> SinglyLinkedList<T> {
        SinglyLinkedList {
            root: ptr::null_mut(),
            leaf: ptr::null_mut(),
            size: 0,
        }
    }

    /// Pushes a new value into the end of the list. O(n) time complexity
    pub fn push(&mut self, data: T) {
        if self.leaf.is_null() {
            self.root = Node::new_mut(data);
            self.leaf = self.root;
        } else {
            unsafe {
                if !(*self.leaf).next.is_null() {
                    return;
                }
                (*self.leaf).next = Node::new_mut(data);
                self.leaf = (*self.leaf).next;
            }
        }

        self.size += 1;
    }

    /// Inserts a new value into the start of the list. O(1) time complexity
    pub fn insert(&mut self, data: T) {
        self.root = Node::new_mut_with_next(data, self.root);

        if self.leaf.is_null() {
            self.leaf = self.root;
        }

        self.size += 1;
    }

    /// Removes the last element of the list. O(n) time complexity
    pub fn pop(&mut self) {
        if self.leaf == self.root {
            unsafe {
                drop(Box::from_raw(self.root));
                self.root = ptr::null_mut();
                self.leaf = ptr::null_mut();
                self.size = 0;
                return;
            }
        }

        let mut current = self.root;

        unsafe {
            // current.next will never be null because we checked if the root
            // is equal to the leaf, confirming us at least 1 next in the list
            while !(*(*current).next).next.is_null() {
                current = (*current).next;
            }

            drop(Box::from_raw((*current).next));
            (*current).next = ptr::null_mut();
            self.leaf = current;
            self.size -= 1;
        }
    }

    /// Removes the first element of the list. O(1) time complexity
    pub fn remove_first(&mut self) {
        if self.size == 0 {
            return;
        }

        if self.leaf == self.root {
            unsafe {
                drop(Box::from_raw(self.root));
                self.root = ptr::null_mut();
                self.leaf = ptr::null_mut();
                self.size = 0;
                return;
            }
        }

        unsafe {
            let new_root = (*self.root).next;
            drop(Box::from_raw(self.root));
            self.root = ptr::null_mut();
            self.root = new_root;
            self.size -= 1;
        }
    }

    /// Removes the first element that matches `data` using PartialEq.
    /// O(n) time complexity
    pub fn remove_data(&mut self, data: T) {
        unsafe {
            if (*self.root).data == data {
                self.remove_first();
                return;
            } else if (*self.leaf).data == data {
                self.pop();
                return;
            }

            let mut past = self.root;
            let mut current = (*self.root).next;

            while (*current).data != data {
                if (*current).next.is_null() {
                    return;
                }
                past = current;
                current = (*current).next;
            }

            // current will always have a next, because we checked for the leaf
            // in the start
            drop(Box::from_raw((*past).next));
            (*past).next = (*current).next;
        }
    }

    /// Removes element at the specified `index`. O(n) time complexity
    pub fn remove_at(&mut self, index: u32) {
        if self.size == 0 {
            return;
        } else {
            if index == 0 {
                self.remove_first();
                return;
            } else if index == self.size - 1 {
                self.pop();
                return;
            }

            let mut current = self.root;
            let mut pos: u32 = 0;

            unsafe {
                while !(*current).next.is_null() && pos < index {
                    if pos == index - 1 {
                        drop(Box::from_raw((*current).next));
                        // current.next.next can be a null pointer, but it is
                        // not a problem
                        (*current).next = (*(*current).next).next;
                        self.size -= 1;
                        return;
                    } else {
                        current = (*current).next;
                        pos += 1;
                    }
                }
            }
        }
    }

    /// Returns the first element's data. O(1) time complexity
    pub fn get_first(&self) -> Option<&T> {
        if self.root.is_null() {
            None
        } else {
            unsafe { Some(&(*self.root).data) }
        }
    }

    /// Returns the last element's data. O(1) time complexity
    pub fn get_last(&self) -> Option<&T> {
        if self.leaf.is_null() {
            None
        } else {
            unsafe { Some(&(*self.leaf).data) }
        }
    }

    /// Returns the element at specified `index`. O(n) time complexity
    pub fn get(&self, index: u32) -> Option<&T> {
        if self.size == 0 {
            None
        } else if index == self.size - 1 {
            self.get_last()
        } else if index == 0 {
            self.get_first()
        } else {
            let mut current = self.root;
            let mut pos: u32 = 0;

            unsafe {
                while !(*current).next.is_null() && pos < index {
                    current = (*current).next;
                    pos += 1;
                }

                if pos != index {
                    None
                } else {
                    Some(&(*current).data)
                }
            }
        }
    }
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data,
            next: ptr::null_mut(),
        }
    }

    fn new_with_next(data: T, next: *mut Node<T>) -> Node<T> {
        Node { data, next }
    }

    fn new_mut(data: T) -> *mut Node<T> {
        Box::into_raw(Box::new(Self::new(data)))
    }

    fn new_mut_with_next(data: T, next: *mut Node<T>) -> *mut Node<T> {
        Box::into_raw(Box::new(Self::new_with_next(data, next)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_first() {
        let list = SinglyLinkedList::new(10);
        assert_eq!(list.get_first(), Some(&10));
        assert_eq!(list.get_last(), Some(&10));
        assert_eq!(list.size, 1);
    }

    #[test]
    fn find_inserted() {
        let mut list = SinglyLinkedList::new(10);
        list.insert(1);
        assert_eq!(list.get_first(), Some(&1));
        assert_eq!(list.get_last(), Some(&10));
        assert_eq!(list.size, 2);
    }

    #[test]
    fn find_last() {
        let mut list = SinglyLinkedList::new(10);
        list.push(20);
        assert_eq!(list.get_first(), Some(&10));
        assert_eq!(list.get_last(), Some(&20));
        assert_eq!(list.size, 2)
    }

    #[test]
    fn find_at() {
        let mut list = SinglyLinkedList::new(10);
        list.push(20);
        list.push(30);
        list.push(40);
        assert_eq!(list.get(0), Some(&10));
        assert_eq!(list.get(1), Some(&20));
        assert_eq!(list.get(2), Some(&30));
        assert_eq!(list.get(3), Some(&40));
    }

    #[test]
    fn find_removed() {
        let mut list = SinglyLinkedList::new(10);
        list.push(20);
        list.push(30);
        list.push(40);
        list.push(50);

        list.remove_first();
        assert_eq!(list.get_first(), Some(&20));
        assert_eq!(list.get(1), Some(&30));
        assert_eq!(list.size, 4);

        list.pop();
        assert_eq!(list.get_last(), Some(&40));
        assert_eq!(list.get(list.size - 2), Some(&30));
        assert_eq!(list.size, 3);

        list.remove_at(1);
        assert_eq!(list.get_first(), Some(&20));
        assert_eq!(list.get_last(), Some(&40));
        assert_eq!(list.get(1), Some(&40));
        assert_eq!(list.size, 2);

        list.remove_data(20);
        assert_eq!(list.get_first(), Some(&40));
        assert_eq!(list.get_last(), Some(&40));
        assert_eq!(list.size, 1);

        list.remove_first();
        assert_eq!(list.get_first(), None);
        assert_eq!(list.get_last(), None);
        assert_eq!(list.size, 0);
    }

    #[test]
    fn find_first_empty_list() {
        let mut list = SinglyLinkedList::<i32>::new_empty();
        assert_eq!(list.size, 0);
        assert_eq!(list.get_first(), None);
        assert_eq!(list.get_last(), None);

        list.push(10);
        assert_eq!(list.size, 1);
        assert_eq!(list.get_first(), Some(&10));
        assert_eq!(list.get_last(), Some(&10));

        list.remove_first();

        list.insert(20);
        assert_eq!(list.get_first(), Some(&20));
        assert_eq!(list.get_last(), Some(&20));
        assert_eq!(list.size, 1);
    }
}
