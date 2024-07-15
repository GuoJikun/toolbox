mod calculator;

use std::env;
use calculator::evaluate_expression;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <expression>", args[0]);
        std::process::exit(1);
    }

    let expression = &args[1];

    match evaluate_expression(expression) {
        Ok(result) => println!("结果: {}", result),
        Err(e) => println!("错误: {}", e),
    }
}
