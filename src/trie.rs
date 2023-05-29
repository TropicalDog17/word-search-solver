use std::collections::HashMap;
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_word: false,
        }
    }
}
impl Default for TrieNode {
    fn default() -> Self {
        TrieNode::new()
    }
}

pub struct Trie {
    root: TrieNode,
}
impl Default for Trie {
    fn default() -> Self {
        Trie::new()
    }
}
impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }
    pub fn from(words: &Vec<&str>) -> Self {
        let mut trie = Trie::new();
        trie.insert_words(words);
        trie
    }
    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            let next_node = current_node.children.entry(c).or_insert(TrieNode::new());
            current_node = next_node;
        }
        current_node.is_word = true;
    }
    pub fn insert_words(&mut self, words: &Vec<&str>) {
        for word in words.to_owned().clone() {
            let mut current_node = &mut self.root;
            for c in word.chars() {
                let next_node = current_node.children.entry(c).or_insert(TrieNode::new());
                current_node = next_node;
            }
            current_node.is_word = true;
        }
    }

    pub fn search(&self, word: &str) -> bool {
        let mut current_node = &self.root;

        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(next_node) => current_node = next_node,
                None => return false,
            }
        }

        current_node.is_word
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut current_node = &self.root;

        for c in prefix.chars() {
            match current_node.children.get(&c) {
                Some(next_node) => current_node = next_node,
                None => return false,
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_start_with() {
        let mut trie = Trie::new();
        trie.insert("Hello");
        trie.insert("Like");
        trie.insert("Interesting");
        trie.insert("K");
        trie.insert("Half");

        assert!(trie.starts_with("H"));
        assert!(trie.starts_with("Li"));
        assert!(trie.starts_with("K"));
        assert!(trie.starts_with(""));
        assert!(trie.starts_with("Interest"));
        assert!(!trie.starts_with("prefix"));
    }
    #[test]
    fn test_insert_and_search() {
        let mut trie = Trie::new();
        trie.insert("Hello");
        trie.insert("Like");
        trie.insert("Interesting");
        trie.insert("K");
        trie.insert("Half");

        assert!(trie.search("Hello"));
        assert!(trie.search("Like"));
        assert!(trie.search("Interesting"));
        assert!(trie.search("K"));
        assert!(trie.search("Half"));
        assert!(!trie.search("prefix"));
    }
    #[test]
    fn test_insert_words() {
        let mut trie = Trie::new();
        let words = vec!["One", "Two", "Three", "Four", "Five"];
        trie.insert_words(&words);
        assert!(trie.search("One"));
        assert!(trie.search("Two"));
        assert!(trie.search("Three"));
        assert!(trie.search("Four"));
        assert!(trie.search("Five"));
        assert!(!trie.search("Six"));
    }
}
