use std::{env::args, fs::write, ops::Sub};

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 4 || args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        println!(
"
ABOUT:
    'calculator' is a simple, platform-agnostic command line calculator that relies on compile-time optimization;

USAGE:
    > calculator <expression>

EXPRESSIONS:
    <number1><operation><number2>

    EXAMPLES:
        1 * 2
        4 + 7
        -15 / 12
        11 - 0

    PLEASE NOTE:
        wildcards like '*' might need to be expressed like '\\*' depending on your shell
");
        std::process::exit(0)
    }

    let number1 = args[1].clone();
    let op = args[2].clone();
    let number2 = args[3].clone();

    let num1: i32 = number1.parse().unwrap();

    let num2: i32 = number2.parse().unwrap();
}
