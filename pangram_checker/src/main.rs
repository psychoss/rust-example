fn main() {
    let characters: Vec<char> = "The quick brown fox jumps over the lazy dog".chars().collect();
    let mut mark: [bool; 26] = [false; 26];
    let ia = 'a' as i32;
    let iA = 'A' as i32;
    let mut h = -1;
    let mut count = 0;
    for c in characters {
        if c >= 'a' {
            if c > 'z' {
                continue;
            }
            h = (c as i32) - ia;


        } else {
            if c < 'A' || c > 'Z' {
                continue;
            }
            h = (c as i32) - iA;
        }
        if !mark[h as usize] {
            
            if count == 25 {
                println!("The statement is a pangram.");
                return;
            }
            mark[h as usize] = true;
            count = count + 1;

        }

    }
    println!("The statement is not pangram.")
}
