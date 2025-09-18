impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        if s.len() < 1 {return 0}
        let mut result = 0;
        let mx = i32::MAX;
        let mn = i32::MIN;
        let mut sgn = 1;
        let mut done = false;
        for c in s.chars(){
            let v = ((c as u8) - b'0') as i32;
            if (c == '-' || c=='+') && !done{
                if sgn == -1 {break}
                if c == '-' {sgn = -1;}
                done = true;
                continue;
            }
            if c == ' ' && !done {continue}

            if 0 <= v && v <= 9{
                if sgn*result <= (mn/10 - v) {return mn}
                if result > (mx/10)  {return mx}
                
                result = result*10 + v;
                done = true;
            }
            else {
                break;
            }
        }

        if 0 > result.signum(){
            if sgn == -1 {result = mn}
            else {result = mx}
        }
        return sgn*(result as i32);
    }
}
