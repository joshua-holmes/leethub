use std::collections::LinkedList;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut r_list = LinkedList::new();
        let mut d_list = LinkedList::new();
        
        for (i, c) in senate.chars().enumerate() {
            match c {
                'R' => r_list.push_back(i),
                'D' => d_list.push_back(i),
                _ => {}
            }
        }
        let mut max_i = senate.len() - 1;
        let result = loop {
            if let Some(r) = r_list.pop_front() {
                if let Some(d) = d_list.pop_front() {
                    max_i += 1;
                    println!("{} {}", r, d);
                    if d < r {
                        d_list.push_back(max_i);
                    } else {
                        r_list.push_back(max_i);
                    }
                } else {
                    break "Radiant";
                }
            } else {
                break "Dire";
            }
        };

        result.to_string()
    }
}