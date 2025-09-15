impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut l = digits.len()-1;
        let mut retain = 0;
        if digits[l] != 9{
            digits[l] += 1;
            return digits;
        }
        retain = 1;
        while retain != 0 {
            if l == 0 && digits[l] == 9{
                digits[l] = retain;
                digits.push(0);
                retain = 0;
            }
            else if digits[l] == 9{
                digits[l] = 0;
                retain = 1;
            }
            else {
                digits[l] += retain;
                retain = 0;
            }
            l -= 1;
        }

        return digits;
    }
}