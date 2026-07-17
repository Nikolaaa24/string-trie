use std::collections::HashMap;
use std::str::Chars;

/// Enum carrying information whether to limit results or not.
/// Passed in the search function.
pub enum ResultLimit {
    Unlimited,
    Limited(usize),
}

pub struct Trie {
    root: Node
}

struct Node {
    children: HashMap<char, Node>,
    is_end: bool,
}

impl Node {
    fn new() -> Node {
        let node = Node { children: HashMap::new(), is_end: false, };
        node
    }

    fn search(&self, word: &str) -> Option<&Node> {

        let mut current_node = self;

        for ch in word.chars() {
            let found_child = current_node.children.get(&ch);
            match found_child {
                Some(node) => {
                    current_node = node;
                }
                None => {
                    return None
                }
            }
        }

        Some(current_node)
    }

    fn collect_words(&self, current_word: &mut String, words: &mut Vec<String>, limit: &ResultLimit) {
        // Push self
        if self.is_end {
            words.push(current_word.clone());
        }

        if let ResultLimit::Limited(len) = limit {
            if words.len() >= *len {
                return;
            }
        }

        // Check all child nodes and push if is_end is true
        for (char, node) in &self.children {
            current_word.push(*char);

            node.collect_words(current_word, words, limit);

            current_word.pop();

            if let ResultLimit::Limited(len) = limit {
                if words.len() >= *len {
                    return;
                }
            }
        }
    }

    fn delete(&mut self, mut it: Chars) -> bool {
        let next_char = it.next();
        match next_char {
            None => {
                if self.is_end
                {
                    self.is_end = false;
                    return true;
                }
                false
            }
            Some(ch) => {
                let mut success = false;

                let found_child = self.children.get_mut(&ch);
                if let Some(node) = found_child {
                    success = node.delete(it);
                    if node.children.is_empty() && !node.is_end
                    {
                        self.children.remove(&ch);
                    }
                }
                if success {
                    return true
                }
                false
            }
        }
    }
}

impl Trie {
    pub fn new() -> Trie {
        let node = Node::new();
        Trie {root: node}
    }

    /// Searches the trie using the prefix string.
    ///
    /// # Example
    /// ```
    /// # use string_trie::{ResultLimit, Trie};
    /// let mut trie = Trie::new();
    /// trie.insert("hello");
    ///
    /// let found = trie.search("h", ResultLimit::Unlimited);
    /// assert_eq!(found[0], String::from("hello"));
    /// ```
    pub fn search(&self, word: &str, limit: ResultLimit) -> Vec<String> {
        let mut vec = Vec::new();
        match self.root.search(word) {
            Some(node) => {
                let mut word_string = String::from(word);
                node.collect_words(&mut word_string, &mut vec, &limit);
                vec
            }
            None => {
                vec
            }
        }
    }

    /// Inserts a string into the trie.
    ///
    /// # Example
    /// ```
    /// # use string_trie::{ResultLimit, Trie};
    /// let mut trie = Trie::new();
    ///
    /// trie.insert("hello");
    ///
    /// let found = trie.search("h", ResultLimit::Unlimited);
    /// assert_eq!(found[0], String::from("hello"));
    /// ```
    pub fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;

        for ch in word.chars() {
            current = current.children.entry(ch).or_insert(Node::new());
        }

        current.is_end = true;
    }

    /// Deletes the string by deleting the node if no children exist or
    /// invalidating the word by setting is_end to false otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// # use string_trie::{ResultLimit, Trie};
    /// let mut trie = Trie::new();
    /// trie.insert("car");
    /// trie.insert("cart");
    ///
    /// trie.delete("car");
    ///
    /// let found = trie.search("car", ResultLimit::Unlimited);
    /// assert_eq!(found[0], String::from("cart"));
    /// ```
    pub fn delete(&mut self, word: &str) -> bool {
        self.root.delete(word.chars())
    }
}