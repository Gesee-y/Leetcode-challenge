impl Solution {
    pub fn longest_palindrome(mut s: String) -> String {
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        let mut one = false;
        let mut r = String::new();
        let chs :Vec<char> = s.chars().collect();

        while i < chs.len() {
            if is_palindrome(&chs, i, j){
                if k < j-i+1 {
                    let v = &s[i..j+1];
                    r = v.to_string();
                    k = j-i+1;
                    one = k > 1;
                }
            }  
            if j >= chs.len()-1 {j=i; i+=1} else {j += 1;}
            
            
        }

        return r;
    }
}

fn is_palindrome(s: &Vec<char>, st:usize, e:usize) -> bool {
    let mut i = st; let mut j = e;
    let l = j-i+1;
    if (l == 2 && s[i] == s[j]) || (l == 1 && s[0] != ' ') {return true}
    

    while i < j {
        if s[i] != s[j] {return false}
        i+=1; j-=1;
    }

    return true;
}
