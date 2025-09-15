impl Solution {
    pub fn climb_stairs(n: i32) -> i64 {
        let mut result :i64 = 0;
        let mut i = 0;
        let mut j = n as i64;
        while j >= i {
            result += combination(i,j);
            i += 1; j -= 1;
        }

        return result;
    }
}

fn combination(i:i64, j:i64) -> i64{
    if i<= 0 {return 1}
    let mut r:i64 = 1;
    let mut m = 1;
    for k in (i+1)..=j {
        r *= k;
        r /= m;
        m += 1;
    }
    for k in m..=(j-i) {
        r /= k;
    }
    return r;
}