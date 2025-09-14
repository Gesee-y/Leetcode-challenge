impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut rev = 0;
        let mut n = x;
        while n > 0{
            rev = rev*10 + (n % 10);
            n /= 10;
        }
        

        return (rev == x);
    }
}
