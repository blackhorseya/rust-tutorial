use std::io;

fn main() {
    loop {
        println!("Type a value: ");
        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line.");

        let mut value: u32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Type a unit: ");
        let mut unit = String::new();

        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");

        let mut unit: char = unit.trim().to_uppercase().chars().last().unwrap();
        if unit != 'F' && unit != 'C' {
            continue;
        }

        println!("Input: {}{}", value, unit);

        if unit == 'C' {
            value = (value * 9 / 5) + 32;
            unit = 'F';
        } else if unit == 'F' {
            value = (value - 32) * 5 / 9;
            unit = 'C';
        }

        println!("Output: {}{}", value, unit)
    }
}


