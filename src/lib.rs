use std::ptr;

pub struct BinarySearchTree<T: PartialOrd> {
    root: *mut Node<T>,
}

impl<T: PartialOrd + Clone> BinarySearchTree<T> {
    pub fn new(data: T) -> BinarySearchTree<T> {
        BinarySearchTree {
            root: Node::new_mut(data),
        }
    }

    unsafe fn add_node(data: T, node: *mut Node<T>) {
        unsafe {
            if data > (*node).data {
                if (*node).right.is_null() {
                    (*node).add_right(data)
                } else {
                    Self::add_node(data, (*node).right)
                }
            } else if data < (*node).data {
                if (*node).left.is_null() {
                    (*node).add_left(data)
                } else {
                    Self::add_node(data, (*node).left)
                }
            }
        }
    }

    pub fn add(&mut self, data: T) {
        unsafe {
            Self::add_node(data, self.root);
        }
    }

    unsafe fn get_node<'a>(data: &T, node: *mut Node<T>) -> Option<&'a T> {
        if node.is_null() {
            None
        } else {
            unsafe {
                if *data > (*node).data {
                    Self::get_node(data, (*node).right)
                } else if *data < (*node).data {
                    Self::get_node(data, (*node).left)
                } else {
                    Some(&(*node).data)
                }
            }
        }
    }

    unsafe fn find_successor_and_delete<'a>(node: *mut Node<T>) -> Option<&'a T> {
        if (*node).right.is_null() {
            return None;
        } else {
            let mut past = node;
            let mut current = (*node).right;

            while !(*current).left.is_null() {
                past = current;
                current = (*current).left;
            }

            if current == (*past).right {
                (*past).delete_right();
            } else {
                (*past).delete_left();
            }

            Some(&(*current).data)
        }
    }

    pub fn get(&self, data: &T) -> Option<&T> {
        unsafe { Self::get_node(data, self.root) }
    }

    unsafe fn delete_node_helper(parent: *mut Node<T>, node: *mut Node<T>) {
        if !(*node).left.is_null() && !(*node).right.is_null() {
            // We know successor will not be `None`, since we
            // checked the left and right values for null ptr
            let successor = Self::find_successor_and_delete(node).unwrap();
            (*node).data = successor.clone();
        } else if (*node).left.is_null() && (*node).right.is_null() {
            (*parent).delete_right();
        } else {
            // Then right.right is not null
            if (*node).left.is_null() {
                (*node).delete_right();
            } else {
                (*node).delete_left();
            }
        }
    }

    unsafe fn delete_node(data: &T, node: *mut Node<T>) {
        if node.is_null() {
            return;
        } else {
            unsafe {
                if *data > (*node).data {
                    let right = (*node).right;
                    if right.is_null() {
                        return;
                    }
                    if *data == (*right).data {
                        Self::delete_node_helper(node, right);
                    } else {
                        Self::delete_node(data, right);
                    }
                } else if *data < (*node).data {
                    let left = (*node).left;
                    if (*node).left.is_null() {
                        return;
                    }
                    if *data == (*(*node).left).data {
                        Self::delete_node_helper(node, left);
                    } else {
                        Self::delete_node(data, (*node).left);
                    }
                }
            }
        }
    }

    pub fn delete(&mut self, data: &T) {
        unsafe {
            if (*self.root).data == *data {
                drop(Box::from_raw(self.root));
                self.root = ptr::null_mut();
            } else {
                Self::delete_node(data, self.root);
            }
        }
    }

    unsafe fn post_order_node<'a, F>(on_find: &mut F, node: *mut Node<T>)
    where
        F: FnMut(&'a T), T: 'a,
    {
        if node.is_null() {
            return;
        }

        Self::post_order_node(on_find, (*node).left);
        Self::post_order_node(on_find, (*node).right);
        on_find(&(*node).data);
    }

    unsafe fn pre_order_node<'a, F>(on_find: &mut F, node: *mut Node<T>)
    where
        F: FnMut(&'a T), T: 'a,
    {
        if node.is_null() {
            return;
        }

        on_find(&(*node).data);
        Self::pre_order_node(on_find, (*node).left);
        Self::pre_order_node(on_find, (*node).right);
    }

    unsafe fn in_order_node<'a, F>(on_find: &mut F, node: *mut Node<T>)
    where
        F: FnMut(&'a T), T: 'a,
    {
        if node.is_null() {
            return;
        }

        Self::in_order_node(on_find, (*node).left);
        on_find(&(*node).data);
        Self::in_order_node(on_find, (*node).right);
    }

    pub fn in_order<'a, F>(&self, on_find: &mut F) where F: FnMut(&'a T), T: 'a {
        unsafe {
            Self::in_order_node(on_find, self.root);
        }
    }

    pub fn pre_order<'a, F>(&self, on_find: &mut F) where F: FnMut(&'a T), T: 'a {
        unsafe {
            Self::pre_order_node(on_find, self.root);
        }
    }

    pub fn post_order<'a, F>(&self, on_find: &mut F) where F: FnMut(&'a T), T: 'a {
        unsafe {
            Self::post_order_node(on_find, self.root);
        }
    }
}

pub struct Node<T: PartialOrd> {
    pub data: T,
    left: *mut Node<T>,
    right: *mut Node<T>,
}

impl<T: PartialOrd> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
        }
    }

    fn new_mut(data: T) -> *mut Node<T> {
        Box::into_raw(Box::new(Self::new(data)))
    }

    fn add_left(&mut self, data: T) {
        if !self.left.is_null() {
            return;
        }
        self.left = Self::new_mut(data)
    }

    fn add_right(&mut self, data: T) {
        if !self.right.is_null() {
            return;
        }
        self.right = Self::new_mut(data)
    }

    fn delete_left(&mut self) {
        unsafe {
            drop(Box::from_raw(self.left));
            self.left = ptr::null_mut()
        }
    }

    fn delete_right(&mut self) {
        unsafe {
            drop(Box::from_raw(self.right));
            self.right = ptr::null_mut()
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn find_root() {
        let tree = BinarySearchTree::new(10);
        assert_eq!(tree.get(&10), Some(&10));
        assert_eq!(tree.get(&9), None);
    }

    #[test]
    fn find_node() {
        let mut tree = BinarySearchTree::new(10);
        tree.add(5);
        tree.add(15);
        assert_eq!(tree.get(&10), Some(&10));
        assert_eq!(tree.get(&5), Some(&5));
        assert_eq!(tree.get(&15), Some(&15));
    }

    #[test]
    fn find_deleted_root() {
        let mut tree = BinarySearchTree::new(10);
        assert_eq!(tree.get(&10), Some(&10));

        tree.delete(&10);
        assert_eq!(tree.get(&10), None);
    }

    #[test]
    fn find_deleted_node() {
        let mut tree = BinarySearchTree::new(10);

        tree.add(20);
        assert_eq!(tree.get(&20), Some(&20));

        tree.delete(&20);
        assert_eq!(tree.get(&20), None);
    }

    #[test]
    fn in_order_check() {
        let mut vals: Vec<i32> = vec![];
        let mut tree = BinarySearchTree::new(10);
        tree.add(5);
        tree.add(1);
        tree.add(9);
        tree.add(15);
        tree.add(30);
        tree.add(11);

        let mut func = |&data| vals.push(data);
        tree.in_order(&mut func);
        assert_eq!(vals.get(0), Some(&1));
        assert_eq!(vals.get(1), Some(&5));
        assert_eq!(vals.get(2), Some(&9));
        assert_eq!(vals.get(3), Some(&10));
        assert_eq!(vals.get(4), Some(&11));
        assert_eq!(vals.get(5), Some(&15));
        assert_eq!(vals.get(6), Some(&30));
    }

    #[test]
    fn pre_order_check() {
        let mut vals: Vec<i32> = vec![];
        let mut tree = BinarySearchTree::new(10);
        tree.add(5);
        tree.add(1);
        tree.add(9);
        tree.add(15);
        tree.add(30);
        tree.add(11);

        let mut func = |&data| vals.push(data);
        tree.pre_order(&mut func);
        assert_eq!(vals.get(0), Some(&10));
        assert_eq!(vals.get(1), Some(&5));
        assert_eq!(vals.get(2), Some(&1));
        assert_eq!(vals.get(3), Some(&9));
        assert_eq!(vals.get(4), Some(&15));
        assert_eq!(vals.get(5), Some(&11));
        assert_eq!(vals.get(6), Some(&30));
    }

    #[test]
    fn post_order_check() {
        let mut vals: Vec<i32> = vec![];
        let mut tree = BinarySearchTree::new(10);
        tree.add(5);
        tree.add(1);
        tree.add(9);
        tree.add(15);
        tree.add(30);
        tree.add(11);

        let mut func = |&data| vals.push(data);
        tree.post_order(&mut func);
        assert_eq!(vals.get(0), Some(&1));
        assert_eq!(vals.get(1), Some(&9));
        assert_eq!(vals.get(2), Some(&5));
        assert_eq!(vals.get(3), Some(&11));
        assert_eq!(vals.get(4), Some(&30));
        assert_eq!(vals.get(5), Some(&15));
        assert_eq!(vals.get(6), Some(&10));
    }
}
