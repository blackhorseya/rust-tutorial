use std::io;

fn main() {
    loop {
        let mut target = String::new();

        io::stdin()
            .read_line(&mut target)
            .expect("Failed to read line");

        let target: u32 = match target.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Target is {target}");

        println!("Fib is {}", fib(target));
        println!("Fib1 is {}", fib1(target));
    }
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    return fib(n - 1) + fib(n - 2);
}

fn fib1(n: u32) -> u32 {
    let mut a: u32 = 0;
    let mut b: u32 = 1;

    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + a;
    }

    return a;
}
