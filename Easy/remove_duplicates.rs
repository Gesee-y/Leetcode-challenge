// My solution for this remove duplicate
// I don't really master rust yet
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut current = 1;
        let mut lst = nums[0];
        for i in 1..nums.len(){
            let elt = nums[i];
            if elt != lst {
                lst = elt;
                nums[current] = lst;
                current += 1;
            }
        }
        return current as i32;
    }
}
