impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        
        let mut s = 0;
        let mut l = nums.len();
        while (l - s) > 1{
            let center = (l+s)/2;
            let elt = nums[center];
            if elt < target {
                s = center;
            }
            else if elt > target{
                l = center;
            }
            else {
                return center as i32;
            }
        }
        if nums[s] >= target {
            return s as i32;
        }
        else {
            return l as i32;
        }
    }
}