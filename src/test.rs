#[cfg(test)]
mod iterator_tests {
    use crate::{Token, TokenIterator};

    #[test]
    fn take_add() {
        let data = vec!['+'];
        let expected = Token::Add;
        let actual = TokenIterator::new(&data).next().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn take_subtract() {
        let data = vec!['-'];
        let expected = Token::Subtract;
        let actual = TokenIterator::new(&data).next().unwrap();
        assert_eq!(expected, actual);
    }
    #[test]
    fn take_divide() {
        let data = vec!['/'];
        let expected = Token::Divide;
        let actual = TokenIterator::new(&data).next().unwrap();
        assert_eq!(expected, actual);
    }
    #[test]
    fn take_multiply() {
        let data = vec!['*'];
        let expected = Token::Multiply;
        let actual = TokenIterator::new(&data).next().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn take_add_then_subtract() {
        let data = vec!['+', '*'];
        let expected = vec![Token::Add, Token::Multiply];
        let actual: Vec<Token> = TokenIterator::new(&data).collect();
        assert_eq!(expected, actual);
    }
    #[test]
    fn take_number() {
        let data = vec!['2'];
        let expected = vec![Token::NonOperator(2)];
        let actual: Vec<Token> = TokenIterator::new(&data).collect();
        assert_eq!(expected, actual);
    }
    #[test]
    fn take_2_digit_number() {
        let data = vec!['2', '0'];
        let expected = vec![Token::NonOperator(20)];
        let actual: Vec<Token> = TokenIterator::new(&data).collect();
        assert_eq!(expected, actual);
    }
    #[test]
    fn take_2_digit_number_with_preceding_addition() {
        let data = vec!['+', '2', '0'];
        let expected = vec![Token::Add, Token::NonOperator(20)];
        let actual: Vec<Token> = TokenIterator::new(&data).collect();
        assert_eq!(expected, actual);
    }
    #[test]
    fn take_2_digit_number_with_preceding_subtraction() {
        let data = vec!['-', '2', '0'];
        let expected = vec![Token::Subtract, Token::NonOperator(20)];
        let actual: Vec<Token> = TokenIterator::new(&data).collect();
        assert_eq!(expected, actual);
    }
}
