


fn largest<T: PartialOrd>(numbers: &[T]) -> Option<&T> {
    let mut result: Option<&T> = None;
    for num in numbers {
        if result.is_none() || num > result.unwrap() {
            result = Some(num);
        }
    }
    result
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let largest = largest(&numbers);
    match largest {
        Some(val) => println!("Largest number in list is: {val}"),
        None => println!("List is empty"),
    }
}
