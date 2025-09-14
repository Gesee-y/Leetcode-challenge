impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sgn = 0;
        let mut lst = 4000;
        let mut res = 0;
        for c in s.chars() {
            let vc = get_value(c);
            if lst < vc{
                sgn = -2;
            }
            res += sgn*lst + vc;
            lst = vc;
            sgn = 0;
        }
      return res;
    }
}

fn get_value(c: char) -> i32{
    match c{
        'I' => return 1,
        'V' => return 5,
        'X' => return 10,
        'L' => return 50,
        'C' => return 100,
        'D' => return 500,
        'M' => return 1000,
        _ => return 0,
    }
}
