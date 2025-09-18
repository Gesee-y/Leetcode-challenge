impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 {return s}
        let mut result = String::new();
        let mut v = Vec::new();
        let mut cc = 0;
        let mut cr = 0;
        let mut i = 0;
        let space = num_rows - 1;
        let chars :Vec<char> = s.chars().collect();

        while i < s.len() {
            let mut c = ' ';
            if cc % space == 0 || (num_rows - cr - 1) == (cc%space) {
                c = chars[i];
                i += 1;
            }

            cr += 1;
            if cr >= num_rows {cr = 0; cc += 1}
            v.push(c);
        }
        
        cc+=1;
        i = 0;

        for i in 0..(num_rows*cc) {
            let x = i%cc;
            let y = i/cc;
            let k = (y + x*num_rows) as usize;
            if k < v.len(){
                let c = v[k];
                if c != ' ' {result.push(c)}
            }
            
        }

        return result;
    }
}