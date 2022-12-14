use std::io;

fn main() {
    println!("Program to print out fibonacci number");

    loop {
        println!("Your number!");
        let mut number = String::new();
        io::stdin()
			.read_line(&mut number)
			.expect("Failed to read line");
            
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        println!("your number is {number}");

        let mut _tmp = 0;
        let mut previous = 0;
        let mut next = 1;
        
        println!("{previous}");
        for _n in 0..number {
            println!("{next}");
            _tmp = next;
            next += previous;
            previous = _tmp;
        }
    }
}
