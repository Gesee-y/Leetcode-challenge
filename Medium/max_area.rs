impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut r = 0;
        let mut left = 0;
        let mut right = height.len()-1;

        while left < right {
            let lv= height[left];
            let rv = height[right];
            let v = lv.min(rv)*((right-left) as i32);
            if v > r {r = v}

            if lv < rv {left += 1}
            else {right -= 1}
        }

        return r;
    }
}