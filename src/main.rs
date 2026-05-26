use std::env::args;

mod addition;
mod division;
mod multiplication;
mod subtraction;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 4 || args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        println!(
"
ABOUT:
    'calculator' is a simple, platform-agnostic command line calculator that relies on compile-time optimization to deliver fast results.

USAGE:
    > calculator <expression>

EXPRESSIONS:
    <number1><operation><number2>

    + => add
    - => subtract
    ° => multiply
    / => divide

    EXAMPLES:
        1 ° 2
        4 + 7
        -15 / 12
        11 - 0

");
        std::process::exit(0)
    }

    let number1 = args[1].clone();
    let op = args[2].clone();
    let number2 = args[3].clone();

    let num1: i32 = number1.parse().unwrap();

    let num2: i32 = number2.parse().unwrap();

    match op.as_str() {
        "°" => println!("{}", multiplication::multiply(num1, num2)),
        "-" => println!("{}", subtraction::subtract(num1, num2)),
        "+" => println!("{}", addition::add(num1, num2)),
        "/" => println!("{}", division::divide(num1, num2)),
        _ => panic!("Invalid operation"),
    }
}
