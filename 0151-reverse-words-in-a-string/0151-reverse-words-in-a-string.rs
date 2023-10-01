impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut words: Vec<&str> = s.split(' ').collect();
        let mut words: Vec<&str> = words.iter().filter(|x| x.len() != 0).map(|x| *x).collect();
        words.reverse();
        words.join(" ")
    }
}