pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    }
}

fn main() {
    let words = &["First", "second", "last"];
    let result: Vec<String> = words
        .iter()
        .map(|s| capitalize_first(s))
        .collect();
    println!("{:?}", &result);
}
