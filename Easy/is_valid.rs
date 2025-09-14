// so we got a string and we need to check it's validity 
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack :Vec<char> = Vec::new();
        let openings = ['(','[','{'];
        let closings = [')',']','}'];
        for c in s.chars(){
            if openings.contains(&c) {
                stack.push(c);
            }
            else if closings.contains(&c) {
                if let Some(lst) = stack.last(){
                    let j = get_index(*lst);
                    let i = get_index(c);
                    if j == i {
                        stack.pop();
                    }
                    else {
                        return false;
                    }
                }
                else {return false;}
            }
        }
        return stack.is_empty();
    }
}

fn get_index(c: char) -> usize{
    match c{
        '(' | ')' => 0,
        '[' | ']' => 1,
        '{'| '}' => 2,
        _ => 0,
    }
                      }
