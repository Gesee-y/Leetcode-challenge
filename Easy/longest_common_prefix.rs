impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs[0].clone();
        for i in 1..strs.len(){
            let mut current = String::new();
            let word = &strs[i];
            
            for j in (0..(std::cmp::min(prefix.len(), word.len()))){
                let c = &prefix[j..j+1];
                if c == &word[j..j+1]{
                    current.push_str(c);
                }
                else {
                    
                    break;
                }
            }
          prefix = current;
        }
        return prefix;
    }
}
