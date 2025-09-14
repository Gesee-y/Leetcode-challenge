// Removing the given value from the array
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {return 0}
        let mut count = nums.len();
        let mut i = 0;
        while i <= count{
            if i == nums.len() {break}
            let mut lst = count-1;
            if nums[i] == val{
                if i == count {}
                while nums[i] == val && lst >= i && lst < count{
                    nums[i] = nums[lst];
                    nums[lst] = val;
                    lst -= 1;
                    count -= 1;
                }
            }
            i += 1;
        }
        return (count) as i32;
    }
}
