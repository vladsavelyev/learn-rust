use std::env;

fn first_word(s: &str) -> &str {
    let mut start: usize = 0;
    let mut end: usize = s.len();
    let mut prev: u8 = b' ';
    for (i, &char) in s.as_bytes().iter().enumerate() {
        if char == b' ' {
            if prev == b' ' {
                start += 1;
                continue
            }
            end = i;
            break
        }
        prev = char;
    }
    &s[start..end]
}

fn second_word(s: &str) -> &str {
    let mut start: usize = s.len();
    let mut end: usize = s.len();
    let mut prev: u8 = b'\n';
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if start == s.len() && prev == b' ' && item != b' ' {
            start = i;
        } else if start != s.len() && prev != b' ' {
            end = i + 1;
            if item == b' ' {
                break;
            }
        }
        prev = item;
    }
    return &s[start..end];
}

fn slices() {
    let args: Vec<String> = env::args().collect();
    
    let s = &args[1].trim();
    
    println!("First word: '{}'", first_word(&s));
    println!("Second word: '{}'", second_word(&s));
}

fn main() {
    slices();
}
