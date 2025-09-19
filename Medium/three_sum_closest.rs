impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let l = nums.len();
        if l == 3 {return nums[0] + nums[1] + nums[2]}
        
        let mut best = i32::MAX;
        let mut sum = 0;
        nums.sort();
        let (mut left, mut right) = (1, l-1);

        for i in 0..l{
            left = i+1; right = l-1;
            while left < right {
                let (a,b,c) = (nums[i], nums[left], nums[right]);
                let s = a + b + c;
                let d = (target - s).abs();
                if s > target {
                    right -= 1;
                }
                else if s < target {
                    left += 1;
                }
                else {
                    return s;
                }

                if d < best{
                    best = d;
                    sum = s;
                }
            }
        }

        return sum;
    }
}