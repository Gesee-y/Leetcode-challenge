impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut current: f64 = 0.0;
        let mut result = (x as f64)/2.0;
        let s = x as f64;
        let mut i = 0;
        while i < 20{
            current = result;
            result = (current + s/current)/2.0;
            i+=1;
        }
        return result.floor() as i32;
    }
}