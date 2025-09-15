impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let l = needle.len();
        for i in (0..haystack.len()) {
            if i+l > haystack.len() {return -1}
            if &haystack[i..i+l] == needle{
                return i as i32;
            }
        }
        return -1;
    }
}