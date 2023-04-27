// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

fn main() {
    let mut ints = vec![0, 4, 6, 4, 2, 0];
    ints.sort();
    println!("{:?}", &ints);

    let len = ints.len();
    let med = if len % 2 == 0 {
        (ints[len / 2 - 1] + ints[len / 2]) / 2
    } else {
        ints[len / 2]
    };
    println!("median: {med}");

    let mut sum = 0;
    for i in &ints {
        sum += i;
    }
    let mean = sum as f32 / len as f32;
    println!("mean: {mean}");
}
