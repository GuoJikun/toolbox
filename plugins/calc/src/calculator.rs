// src/calculator.rs
pub fn evaluate_expression(expression: &str) -> Result<f64, String> {
    let mut num_stack = Vec::new();
    let mut op_stack = Vec::new();
    let mut current_number = String::new();

    let mut chars = expression.chars().peekable();

    while let Some(&ch) = chars.peek() {
        if ch.is_digit(10) || ch == '.' {
            current_number.push(ch);
            chars.next();
        } else {
            if !current_number.is_empty() {
                num_stack.push(current_number.parse::<f64>().map_err(|_| "无效的数字".to_string())?);
                current_number.clear();
            }

            match ch {
                '+' | '-' | '*' | '/' => {
                    while let Some(&top_op) = op_stack.last() {
                        if precedence(top_op) >= precedence(ch) {
                            let result = apply_operator(op_stack.pop().unwrap(), &mut num_stack)?;
                            num_stack.push(result);
                        } else {
                            break;
                        }
                    }
                    op_stack.push(ch);
                    chars.next();
                },
                '(' => {
                    op_stack.push(ch);
                    chars.next();
                },
                ')' => {
                    while let Some(top_op) = op_stack.pop() {
                        if top_op == '(' {
                            break;
                        }
                        let result = apply_operator(top_op, &mut num_stack)?;
                        num_stack.push(result);
                    }
                    chars.next();
                },
                ' ' => {
                    chars.next();
                },
                _ => {
                    return Err("无效的字符".to_string());
                }
            }
        }
    }

    if !current_number.is_empty() {
        num_stack.push(current_number.parse::<f64>().map_err(|_| "无效的数字".to_string())?);
    }

    while let Some(op) = op_stack.pop() {
        let result = apply_operator(op, &mut num_stack)?;
        num_stack.push(result);
    }

    if num_stack.len() == 1 {
        Ok(num_stack.pop().unwrap())
    } else {
        Err("无效的表达式".to_string())
    }
}

fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn apply_operator(op: char, num_stack: &mut Vec<f64>) -> Result<f64, String> {
    if num_stack.len() < 2 {
        return Err("无效的操作数堆栈".to_string());
    }
    let b = num_stack.pop().unwrap();
    let a = num_stack.pop().unwrap();

    match op {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            if b == 0.0 {
                Err("除数不能为零".to_string())
            } else {
                Ok(a / b)
            }
        },
        _ => Err("无效的运算符".to_string()),
    }
}
