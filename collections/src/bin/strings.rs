// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn main() {
    let s = "Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!";

    let mut new_string = String::new();
    let mut cons = 'h';

    for w in s.split_whitespace() {
        'chars: for (i, char) in w.chars().enumerate() {
            if i == 0 && char.is_alphabetic() && char.is_lowercase() {
                for vowel in ['a', 'e', 'o', 'u', 'i'] {
                    if char == vowel {
                        new_string.push(char);
                        continue 'chars;
                    }
                }
                cons = char;
            } else {
                new_string.push(char);
            }
        }
        new_string.push_str(&format!("-{cons}ay "));
        cons = 'h';
    }

    dbg!(new_string);
}