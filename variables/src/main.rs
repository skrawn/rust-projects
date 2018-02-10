
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0/9.0
}

fn fibonacci(num: u32) -> u32 {
    if num == 0 {
        0
    } else if num == 1 {
        1
    } else if num == 2 {
       1
    } else {
        
    }

use std::io;

fn main() {
    let temp_f: f64;     

    loop {
        println!("Enter a temperature in Fahrenheit");
        let mut temp_str = String::new();
        io::stdin().read_line(&mut temp_str).expect("Failed to read line");
        temp_f = match temp_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break; 
    }

    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{} F = {} C", temp_f, temp_c);

    let fib_start: u32;
    loop {
        println!("Enter the number of interations to run on the Fibonacci sequence");
        let mut fib_str = String::new();
        io::stdin().read_line(&mut fib_str).expect("Failed to read line");
        fib_start = match fib_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }

    println!("The {} element in the Fibonacci sequence is {}", fib_start, fibonacci(fib_start));

}
