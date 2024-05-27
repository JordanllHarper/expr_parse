mod test;
fn main() {
    println!("Hello, world!");
}

fn parse_single_number(input: &str) -> i32 {
    let num = input.chars().collect::<Vec<_>>();
    todo!()
}
pub fn is_operator(char: char) -> bool {
    matches!(char, '+' | '-' | '/' | '*')
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    NonOperator(i32),
}

#[derive(Debug)]
struct TokenIterator {
    expression: Vec<char>,
    index: usize,
}

impl TokenIterator {
    fn new(expression: &[char]) -> Self {
        Self {
            expression: expression.to_vec(),
            index: 0,
        }
    }
}

impl Iterator for TokenIterator {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let current_index = self.index;
        let characters_from_index = self.expression.split_at(current_index).1.iter();
        let mut number_buffer = String::new();
        for character in characters_from_index {
            self.index += 1;
            let token = match_operator_with_token(character);
            if token.is_none() {
                // is some number - accumulate
                number_buffer = format!("{}{}", number_buffer, character);
            } else {
                return token;
            }
        }
        if number_buffer.is_empty() {
            None
        } else {
            Some(Token::NonOperator(number_buffer.parse().unwrap()))
        }
    }
}
fn match_operator_with_token(char: &char) -> Option<Token> {
    match char {
        '+' => Some(Token::Add),
        '-' => Some(Token::Subtract),
        '*' => Some(Token::Multiply),
        '/' => Some(Token::Divide),
        _ => None,
    }
}
