impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut res = String::new();
        let mut carry = '0';
        let s1 = if a.len() > b.len() {a.clone()} else {b.clone()};
        let s2 : Vec<char>=  if a.len() > b.len() {b.chars().collect()} else {a.chars().collect()};
        let mut i =s2.len()-1;
            
        for c1 in s1.chars().rev(){
            let c2 = if 0 <= i && i < s2.len() {s2[i]} else {'0'};
            
            if c1 == c2{
                if c1 == '0'{
                    res.push(carry);
                    carry = '0';
                }
                else{
                    res.push(if carry == '1' {'1'} else {'0'});
                    carry = '1';
                }
            }
            else if carry == '1'{
                res.push('0');
            }
            else {
                res.push('1');
            }
            i-= 1;
        }
        if carry == '1' {res.push(carry);}
        return res.chars().rev().collect();
    }
}