use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() < 2 {return s.len() as i32}
        let mut charmap : HashMap<char, i32> = HashMap::new();
        let mut r = 0;
        let mut st = 0;
        let mut e = 0;
        for c in s.chars(){
            
            if let Some(&k) = charmap.get(&c){
                if k >= st {st = k + 1}
            }
            charmap.insert(c, e);

            e += 1;
            let current = e - st;
            if current > r {r = current}
        }

        return r as i32;
    }
}
