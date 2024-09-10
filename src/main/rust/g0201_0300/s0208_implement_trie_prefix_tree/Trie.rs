// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #String #Hash_Table #Design #Trie
// #Level_2_Day_16_Design #Udemy_Trie_and_Heap
// #Big_O_Time_O(word.length())_or_O(prefix.length())_Space_O(N)
// #2024_09_10_Time_13_ms_(90.59%)_Space_9_MB_(97.65%)

use std::collections::HashMap;

struct Trie {
    tree: HashMap<char, Vec<String>>
}


/**
  * `&self` means the method takes an immutable reference
  * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Trie{tree:HashMap::new()}
    }

    fn insert(&mut self, word: String) {
        self.tree.entry(word.chars().next().unwrap()).or_default().push(word);
    }

    fn search(&self, word: String) -> bool {
        if let Some(vals) = self.tree.get(&word.chars().next().unwrap()) {
            return vals.contains(&word)
        }
        false
    }

    fn starts_with(&self, prefix: String) -> bool {
        if let Some(vals) = self.tree.get(&prefix.chars().next().unwrap()) {
            return vals.iter().any(|word| word.starts_with(&prefix))
        }
        false
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
