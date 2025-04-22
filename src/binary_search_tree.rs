use std::ptr;

/// Binary Tree most used when you need to quickly search through a set of
/// ordered values.
///
/// Binary Search Trees (BST) are great for search in ordered sets because it
/// has logarithmic time complexity (O(log n)) when it comes to search, insert and
/// deletion.
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
/// use dsa_abc::binary_search_tree::BinarySearchTree;
///
/// // Creates the BST with an initial value of 10
/// let mut tree = BinarySearchTree::new(10);
/// // Adds a new value following BST's ordering rules
/// tree.add(5);
/// assert_eq!(tree.get(&5), Some(&5));
/// ```
///
/// #### Deleting an element
///
/// ```
/// use dsa_abc::binary_search_tree::BinarySearchTree;
/// 
/// let mut tree = BinarySearchTree::new(10);
/// tree.add(5);
/// assert_eq!(tree.get(&5), Some(&5));
/// 
/// // deletes the node that contains a value that is equal to 5
/// tree.delete(&5);
/// assert_eq!(tree.get(&5), None);
/// ```
///
/// #### Traversal
///
/// ```
/// use dsa_abc::binary_search_tree::BinarySearchTree;
///
/// let mut tree = BinarySearchTree::new(10);
/// tree.add(5);
/// tree.add(15);
///
/// let mut on_find = |&data| println!("{}", data);
/// // Prints to console:
/// // 5
/// // 10
/// // 15
/// tree.in_order(&mut on_find);
/// ```
/// 
pub struct BinarySearchTree<T: PartialOrd> {
    root: *mut Node<T>,
}

impl<T: PartialOrd + PartialEq + Clone> BinarySearchTree<T> {
    /// Create a new BST with an initial data as root
    pub fn new(data: T) -> BinarySearchTree<T> {
        BinarySearchTree {
            root: Node::new_mut(data),
        }
    }

    /// Add a node recursively with data. If data already exists in tree, ignore.
    /// Node must not be root
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

    /// Add a node to the BST using `data`. If data already exists in tree,
    /// ignore. O(log n) time complexity, O(1) space complexity
    pub fn add(&mut self, data: T) {
        if self.root.is_null() {
            self.root = Node::new_mut(data);
        } else {
            unsafe {
                Self::add_node(data, self.root);
            }
        }
    }

    /// Get node value from `data`. Primarily used to check if a given data is
    /// present in the BST
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

    /// Finds a successor to a node, deletes it and returns its value for later
    /// replacement in another node
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

    /// Get a node value for `data` if a node exists with this data. Primarily
    /// used to check if a given data is present in the BST. O(log n) time
    /// complexity, O(1) space complexity
    pub fn get(&self, data: &T) -> Option<&T> {
        unsafe { Self::get_node(data, self.root) }
    }

    /// Helper function to delete node
    unsafe fn delete_node_helper(parent: *mut Node<T>, node: *mut Node<T>, right: bool) {
        if !(*node).left.is_null() && !(*node).right.is_null() {
            // We know successor will not be `None`, since we
            // checked the left and right values for null ptr
            let successor = Self::find_successor_and_delete(node).unwrap();
            (*node).data = successor.clone();
        } else if (*node).left.is_null() && (*node).right.is_null() {
            if right {
                (*parent).delete_right();
            } else {
                (*parent).delete_left();
            }
        } else {
            // Then right.right is not null
            if (*node).left.is_null() {
                (*node).delete_right();
            } else {
                (*node).delete_left();
            }
        }
    }

    /// Deletes a node. Does not worth if node is root
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
                        Self::delete_node_helper(node, right, true);
                    } else {
                        Self::delete_node(data, right);
                    }
                } else if *data < (*node).data {
                    let left = (*node).left;
                    if left.is_null() {
                        return;
                    }
                    if *data == (*left).data {
                        Self::delete_node_helper(node, left, false);
                    } else {
                        Self::delete_node(data, left);
                    }
                }
            }
        }
    }

    /// Deletes a node. O(log n) time complexity, O(1) space complexity
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
        F: FnMut(&'a T),
        T: 'a,
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
        F: FnMut(&'a T),
        T: 'a,
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
        F: FnMut(&'a T),
        T: 'a,
    {
        if node.is_null() {
            return;
        }

        Self::in_order_node(on_find, (*node).left);
        on_find(&(*node).data);
        Self::in_order_node(on_find, (*node).right);
    }

    /// In order traversal with `on_find` callback when each node is found
    pub fn in_order<'a, F>(&self, on_find: &mut F)
    where
        F: FnMut(&'a T),
        T: 'a,
    {
        unsafe {
            Self::in_order_node(on_find, self.root);
        }
    }

    /// Pre order traversal with `on_find` callback when each node is found
    pub fn pre_order<'a, F>(&self, on_find: &mut F)
    where
        F: FnMut(&'a T),
        T: 'a,
    {
        unsafe {
            Self::pre_order_node(on_find, self.root);
        }
    }

    /// Post order traversal with `on_find` callback when each node is found
    pub fn post_order<'a, F>(&self, on_find: &mut F)
    where
        F: FnMut(&'a T),
        T: 'a,
    {
        unsafe {
            Self::post_order_node(on_find, self.root);
        }
    }
}

pub struct Node<T: PartialOrd> {
    data: T,
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

#[cfg(test)]
mod tests {
    use super::*;

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

        tree.add(5);
        assert_eq!(tree.get(&5), Some(&5));

        tree.delete(&5);
        assert_eq!(tree.get(&5), None);

        tree.add(15);
        tree.delete(&15);
        assert_eq!(tree.get(&15), None);

        tree.add(5);
        tree.add(1);
        tree.add(9);
        tree.delete(&5);
        assert_eq!(tree.get(&5), None);

        let mut vals: Vec<i32> = vec![];
        let mut on_find = |&data| vals.push(data);
        tree.in_order(&mut on_find);
        assert_eq!(vals.get(0), Some(&1));
        assert_eq!(vals.get(1), Some(&9));
        assert_eq!(vals.get(2), Some(&10));
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
