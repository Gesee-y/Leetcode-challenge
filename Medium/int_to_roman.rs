impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut s = String::new();
        while num >= 1000{
            s.push('M');
            num -= 1000;
        }

        if num >= 900 {
            num -= 900;
            s.push_str("CM");
        }
        else if num >= 500 {
            num -= 500; 
            s.push('D');
        }
        else if num >= 400 {
            num -= 400;
            s.push_str(&"CD");
        }
        
        while num >= 100 {
            num -= 100;
            s.push('C');
        }

        if num >= 90 {
            num -= 90;
            s.push_str("XC");
        }
        else if num >= 50 {
            num -= 50; 
            s.push('L');
        }
        else if num >= 40 {
            num -= 40;
            s.push_str("XL");
        }

        while num >= 10 {
            num -= 10;
            s.push('X');
        }

        if num == 9 {
            num -= 9;
            s.push_str("IX");
        }
        else if num >= 5{
            num -= 5; 
            s.push('V');
        }
        else if num == 4 {
            num -= 4;
            s.push_str("IV");
        }

        while num >= 1 {
            num -= 1;
            s.push('I');
        }

        return s;
    }
}