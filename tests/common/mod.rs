use shank_rust::Token;

pub fn compare_tokens(left: Vec<Token>, right: Vec<Token>) {
    for (lt, rt) in left.iter().zip(right.iter()) {
        assert_eq!(lt, rt);
    }
}

#[macro_export]
macro_rules! tokenize {
    ( $t:ident, $s:literal, $l:expr ) => {
        Token::new_with_value(TokenType::$t, $s, $l)
    };

    ( $t:ident, $e:expr ) => {
        Token::new(TokenType::$t, $e)
    };

    ( $t:ident ) => {
        Token::new(TokenType::$t, 1)
    }
}


