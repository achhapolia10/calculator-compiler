use std::io;

/**
 * Incresing order of presedence
 * number ->0
 * +,-  -> 1,2
 * *,/ -> 3,4
 * ^ -> 5
 * unary +,- -> 6,7
 * (,) -> 8,9
 */

fn tokenizer(expression: String) -> (Vec<String>, Vec<u8>) {
    let mut current_continue_token = String::from("");
    let mut token_type: Vec<u8> = Vec::new();
    let mut continue_forward = false;
    let mut tokens: Vec<String> = Vec::new();
    let mut count = 0;
    let mut is_num_pushed_last = false;
    let mut paran_encountered = 0;
    for character in expression.chars() {
        count = count + 1;
        let ch = character as u8;
        let length = current_continue_token.chars().count();
        // Check for numbers
        if ch == 32 || ch == 10 || ch == 13 {
            if continue_forward {
                tokens.push(current_continue_token.clone());
                token_type.push(0);
                is_num_pushed_last = true;
                current_continue_token = String::from("");
                continue_forward = false;
            }
            continue;
        }
        if ch <= 57 && ch >= 48 {
            if continue_forward {
                current_continue_token.push(character);
                continue_forward = true;
                continue;
            }
            if length > 0 {
                tokens.push(current_continue_token.clone());
                is_num_pushed_last = true;
                token_type.push(0);
            }
            current_continue_token = String::from("");
            current_continue_token.push(character);
            is_num_pushed_last = true;
            continue_forward = true;
            continue;
        }
        if ch <= 47 && ch >= 40 || ch == 94 {
            if length > 0 {
                tokens.push(current_continue_token.clone());
                is_num_pushed_last = true;
                token_type.push(0);
            }
            match token_type.last().clone() {
                None => {
                    if ch != 43 && ch != 45 && ch != 40 {
                        println!("Illegal start of expression: {}:{}", character, count);
                        std::process::exit(1);
                    }
                }
                Some(a) => {
                    if ch == 42 || ch == 43 || ch == 45 || ch == 47 || ch == 94 {
                        if *a == 1 || *a == 2 || *a == 3 || *a == 4 || *a == 5 {
                            if ch != 43 && ch != 45 {
                                println!(
                                    "Error Unknown token repetitive operations: {}:{}",
                                    character, count
                                );
                                std::process::exit(1);
                            }
                        }
                    }
                }
            };
            tokens.push(character.to_string());
            match ch {
                40 => {
                    token_type.push(8);
                    paran_encountered += 1;
                } // (
                41 => {
                    token_type.push(9);
                    if paran_encountered == 0 {
                        println!("Missing opening paranthesis: {}:{}", character, count);
                        std::process::exit(1);
                    }
                    paran_encountered -= 1;
                } // )
                42 => token_type.push(3), // *
                43 => {
                    if is_num_pushed_last {
                        token_type.push(1);
                    } else {
                        token_type.push(6);
                    };
                } // +
                45 => {
                    if is_num_pushed_last {
                        token_type.push(2);
                    } else {
                        token_type.push(7);
                    };
                } // -
                47 => token_type.push(4), // /
                94 => token_type.push(5), // ^
                _ => {
                    println!("Error Unknown token: {}:{}", character, count);
                    std::process::exit(1);
                }
            };
            is_num_pushed_last = false;
            current_continue_token = String::from("");
            continue;
        }
        println!("Error Unknown token: {}:{}", character, count);
        std::process::exit(1);
    }
    return (tokens, token_type);
}

fn parser(tokens: &Vec<String>, token_types: &Vec<u8>) -> (Vec<String>, Vec<u8>) {
    let mut postfix_tokens: Vec<String> = Vec::new();
    let mut postfix_token_types: Vec<u8> = Vec::new();
    let mut temp_stack: Vec<String> = Vec::new();
    let mut temp_stack_type: Vec<u8> = Vec::new();
    let mut last_inserted_type: u8 = 0;
    let mut temp_last_inserted_type: u8 = 0;
    let mut current_unary: u8 = 100;
    for it in tokens.iter().zip(token_types.iter()) {
        let (token, token_type) = it;
        if *token_type == 0 {
            if current_unary != 100 {
                if current_unary == 6 {
                    postfix_tokens.push(format!("+{}", token.clone()));
                    current_unary = 100
                } else if *token_type == 7 {
                    postfix_tokens.push(format!("-{}", token.clone()));
                    current_unary = 100
                }
            } else {
                postfix_tokens.push(token.clone());
            }
            postfix_token_types.push(0);
            last_inserted_type = 0;
            continue;
        }
        if *token_type == 6 || *token_type == 7 {
            if current_unary == 100 {
                current_unary = *token_type;
            } else {
                if *token_type != current_unary {
                    current_unary = 7;
                } else {
                    current_unary = 6;
                }
            };
            continue;
        }
        if *token_type == 1
            || *token_type == 2
            || *token_type == 3
            || *token_type == 4
            || *token_type == 5
        {
            if *token_type > temp_last_inserted_type || temp_last_inserted_type == 8 {
                temp_stack.push(token.clone());
                temp_stack_type.push(*token_type);
                temp_last_inserted_type = *token_type;
            } else {
                while temp_stack.len() > 0 {
                    let temp_stack_type_elem = temp_stack_type.pop().expect("empty stack");
                    let temp_stack_elem = temp_stack.pop().expect("empty stack");
                    if temp_stack_type_elem == 8 {
                        break;
                    }
                    postfix_token_types.push(temp_stack_type_elem);
                    postfix_tokens.push(temp_stack_elem);
                }
                temp_stack.push(token.clone());
                temp_stack_type.push(*token_type);
                temp_last_inserted_type = *token_type;
            }
        }
        if *token_type == 8 {
            temp_stack.push(token.clone());
            temp_stack_type.push(*token_type);
            temp_last_inserted_type = *token_type;
        }
        if *token_type == 9 {
            while temp_stack.len() > 0 {
                let temp_stack_type_elem = temp_stack_type.pop().expect("empty stack");
                let temp_stack_elem = temp_stack.pop().expect("empty stack");
                if temp_stack_type_elem == 8 {
                    break;
                }
                postfix_token_types.push(temp_stack_type_elem);
                postfix_tokens.push(temp_stack_elem);
            }
        }
    }

    while temp_stack.len() > 0 {
        postfix_token_types.push(temp_stack_type.pop().expect("empty stack"));
        postfix_tokens.push(temp_stack.pop().expect("empty stack"));
    }
    return (postfix_tokens, postfix_token_types);
}

fn execute(tokens: &Vec<String>, token_types: &Vec<u8>) -> f32 {
    let mut temp_stack: Vec<String> = Vec::new();
    for it in tokens.iter().zip(token_types.iter()) {
        let (token, token_type) = it;
        if *token_type == 0 {
            if token.clone() == "" {
                continue;
            }
            temp_stack.push(token.clone());
        } else {
            let op1: f32 = temp_stack.pop().unwrap().parse().unwrap();
            let op2: f32 = temp_stack.pop().unwrap().parse().unwrap();
            println!("operator {},{}", op1, op2);
            let mut res: f32 = 0.0;
            match *token_type {
                1 => {
                    res = op2 + op1;
                }
                2 => {
                    res = op2 - op1;
                }
                3 => {
                    res = op2 * op1;
                }
                4 => {
                    res = op2 / op1;
                }
                5 => {
                    res = op2.powf(op1);
                }
                _ => {}
            }
            temp_stack.push(res.to_string());
        }
    }
    return temp_stack.pop().unwrap().parse().unwrap();
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");
    input.push(' ');
    let (tokens, token_types) = tokenizer(input);
    let (postfix_tokens, postfix_token_types) = parser(&tokens, &token_types);
    let result = execute(&postfix_tokens, &postfix_token_types);
    println!("Tokenized: {:?}, {:?}", tokens, token_types);
    println!("Parsed: {:?}, {:?}", postfix_tokens, postfix_token_types);
    println!("Result: {}", result);
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1);
}
