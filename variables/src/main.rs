fn fib(n: usize) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn fib_loop(n: usize) -> u32 {
    let mut mem: Vec<u32> = Vec::new();
    mem.push(0);
    mem.push(1);
    for i in 2..n + 1 {
        mem.push(mem[i - 1] + mem[i - 2]);
    }
    mem[n]
}

fn main() {
    // let mut s = String::new();
    // stdin().read_line(&mut s).unwrap();

    // let f: i32 = s.trim().parse().expect("Must be a number");

    // let c: i32 = (f - 32) * 5 / 9;

    // println!("Farenheit: {f}, celcius: {c}")

    let mut i: usize = 0;
    while i < 10 {
        println!("{i}: {}, {}", fib(i), fib_loop(i));
        i += 1;
    }
}
