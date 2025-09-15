impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut l = 0;
        for c in s.chars().rev(){
            if c == ' ' && l != 0{
                return l as i32;
            }
            else if c != ' '{
                l += 1;
            }
        }
        return l as i32;
    }
}