# 📚 Rust DSA Library
A collection of core Data Structures and Algorithms implemented in Rust.
Currently includes a fully functional and tested Binary Search Tree (BST) with support for insert, delete, search, and traversal operations.

## 🚀 Features
- ✅ Binary Search Tree (BST)
    - Insert
    - Delete
    - Search
    - In-order, Pre-order, Post-order traversals
- ✅ Singly Linked List
    - Insert
    - Delete
    - Get by index and data
- 📈 Logarithmic time complexity for insert/search/delete in balanced trees
- 🧪 Thoroughly tested with unit tests
- 🦀 Unsafe Rust for raw pointer manipulation (performance reasons)

## 📦 Installation
Clone this repository and include it in your workspace or build it as a Rust library:

git clone https://github.com/lucasmelodev1/dsa_abc
cd dsa_abc
cargo build

## 📘 Usage Example

```rust
use dsa_abc::BinarySearchTree;

fn main() {
    let mut bst = BinarySearchTree::new(10);
    bst.add(5);
    bst.add(15);

    if let Some(val) = bst.get(&5) {
        println!("Found: {}", val);
    }

    // Traversal example
    let mut values = vec![];
    bst.in_order(&mut |v| values.push(*v));
    println!("In-order values: {:?}", values);
}
```

## ✅ Tests

Run tests using:
```bash
cargo test
```

Tests include:
- Basic insert/search/delete
- Deleting root and internal nodes
- Verifying correct order in traversals (in-order, pre-order, post-order)

## 📂 Structure

```file
src/
├── lib.rs        # Core implementation
└── ...
```

## 🔧 Planned Features

This crate aims to be an educational toolkit for practicing and learning DSA in Rust. Upcoming additions include:
- ✅ Binary Search Tree
- ✅ Singly Linked List
- ⏳ Doubly Linked List
- ⏳ AVL Tree
- ⏳ Red-Black Tree
- ⏳ Hash Table
- ⏳ Sorting Algorithms (Merge, Quick, Bubble)
- ⏳ Heap

## ⚠️ Safety Disclaimer

This library uses unsafe code to manually manage pointers for educational purposes. Check the tests section to see if they are enough for your use case and, if not enough, please contact me and I will update them.

## 📄 License
MIT License
