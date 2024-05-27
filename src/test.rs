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
}
