use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];

        let map = HashMap::from([
            ('}', '{'),
            (']', '['),
            (')', '(')
        ]);

        for c in s.chars() {
            if map.contains_key(&c) {
                if stack.is_empty() || stack.pop().unwrap() != *map.get(&c).unwrap() {
                    return false;
                }
            } else {
                stack.push(c);
            }
        }

        stack.is_empty()
    }
}