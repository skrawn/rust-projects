
use std::io;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0/9.0
}

fn fibonacci(num: u32) -> u32 {
    if num == 0 || num == 1 {
        0
    } else if num == 2 {
        1
    } else if num == 3 {
        1
    } else {
        let mut num_1 = 1;
        let mut num_2 = 1;
        let mut result = 0;

        for _x in 0..(num - 3) {
            result = num_1 + num_2;
            num_2 = num_1;
            num_1 = result;
        }
        result
    }
}

fn print_twelve_days_of_christmas_lyrics() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "nineth", "tenth", "eleventh", "twelvth"];
    let gifts = ["A partridge in a pear tree", "Two turtle doves, and", 
        "Three french hens", "Four colly birds", "Five gold rings",
        "Six geese a-laying", "Seven swans a-swimming",
        "Eight maids a-milking", "Nine ladies dancing",
        "Ten lords a-leaping", "Eleven pipers piping", 
        "Twelve drummers drumming"];

    println!("");
    for x in 0..12 {
        println!("On the {} day of Christmas my true love gave to me", 
            days[x]);
        for y in 0..x {
            println!("{}", gifts[x - y]);
        }
        println!("{}", gifts[0]);
        println!("");
    }
}

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

    println!("Element {} in the Fibonacci sequence is {}", fib_start, fibonacci(fib_start));

    print_twelve_days_of_christmas_lyrics();
}
