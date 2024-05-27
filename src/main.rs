mod test;
fn main() {
    println!("Hello, world!");
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
        let current_character = self.expression.get(self.index)?;

        if current_character.is_numeric() {
            //NOTE: This does NOT apply semantic meaning to the number, we don't know
            //if it's positive or negative
            // This is for the parsing stage
            let conseq_numbers = self
                .expression
                .split_at(current_index)
                .1
                .iter()
                .take_while(|it| it.is_numeric())
                .collect::<String>();
            self.index += conseq_numbers.len();
            let number = conseq_numbers.parse::<i32>().unwrap();

            Some(Token::NonOperator(number))
        } else {
            self.index = current_index + 1;
            match_operator_with_token(current_character)
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
