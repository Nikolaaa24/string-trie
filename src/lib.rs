//! A simple trie (prefix trie) for storing and searching strings.
//!
//! Supports prefix search with an optional result limit.
//!
//! # Example
//!
//! ```
//! use string_trie::{Trie, ResultLimit};
//!
//! let mut trie = Trie::new();
//! trie.insert("hello");
//! trie.insert("help");
//! trie.insert("bye");
//!
//! trie.delete("bye");
//!
//! let results = trie.search("hel", ResultLimit::Unlimited);
//! assert_eq!(results.len(), 2);
//! ```

mod string_trie;

pub use string_trie::{Trie, ResultLimit};