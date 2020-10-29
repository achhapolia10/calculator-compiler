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
    for character in expression.chars() {
        count = count + 1;
        let ch = character as u8;
        let length = current_continue_token.chars().count();
        // Check for numbers
        if ch == 32 || ch == 10 {
            if continue_forward {
                tokens.push(current_continue_token.clone());
                is_num_pushed_last = true;
                current_continue_token = String::from("");
                continue_forward = false;
                token_type.push(0);
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
                token_type.push(0)
            }
            tokens.push(character.to_string());
            match ch {
                40 => token_type.push(8), // (
                41 => token_type.push(9), // )
                42 => token_type.push(3), // *
                43 => {
                    if is_num_pushed_last {
                        token_type.push(1);
                    } else {
                        token_type.push(6);
                    }
                } // +
                45 => {
                    if is_num_pushed_last {
                        token_type.push(2);
                    } else {
                        token_type.push(7);
                    }
                } // -
                47 => token_type.push(4), // /
                94 => token_type.push(5), // ^
                _ => {
                    println!("Error Unknown token: {}:{}", character, count);
                    std::process::exit(1);
                }
            }
            is_num_pushed_last = false;
            current_continue_token = String::from("");
            continue;
        }
        println!("Error Unknown token: {}:{}", character, count);
        std::process::exit(1);
    }
    return (tokens, token_type);
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");
    input.push(' ');
    let (tokens, token_types) = tokenizer(input);
    println!("{:?}, {:?}", tokens, token_types);
}
