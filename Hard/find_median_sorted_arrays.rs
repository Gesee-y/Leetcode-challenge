impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        
        let mut result :Vec<i32> = Vec::new();
        let mut r = 0.0;
        let mut i1 :usize = 0;
        let mut i2 :usize = 0;
        let s1 = nums1.len();
        let s2 = nums2.len();
        let l = (s1+s2)/2;
        while i1 < s1 && i2 < s2 && (i1 + i2) <= l{
            let v1 = nums1[i1];
            let v2 = nums2[i2];
            if v1 <= v2{
                result.push(v1);
                i1 += 1;
            }
            if v2 <= v1{
                result.push(v2);
                i2 += 1;
            }
        }
        if (i1 + i2) <= l {
            if i1 < s1 {
                for i in i1..s1{
                    result.push(nums1[i]);
                }
            }
            else if i2 < s2 {
                for i in i2..s2{
                    result.push(nums2[i]);
                }
            }
        }

        if (s1+s2) % 2 == 1{ return result[l] as f64}
        return ((result[l-1] as f64) + (result[l] as f64))/2.0;
    }
}

