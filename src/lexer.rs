use crate::token::{Token, TokenType};
use std::str;

pub struct Lexer<'a> {
    source: &'a [u8],
    tokens: Vec<Token<'a>>,
    current: usize,
    start: usize,
}

impl<'a> Lexer<'a> {
    #[must_use] pub const fn new(source: &'a str) -> Self {
        Self {
            source: source.as_bytes(),
            tokens: vec![],
            current: 0,
            start: 0,
        }
    }

    pub fn scan(&mut self) {
       while !self.is_at_end() {
           self.start = self.current;
           let b = self.advance();

           match b {
               b'#' => self.header(),
               _ => (),
           }
       } 
       self.add_token(TokenType::EOF, "");
    }

    fn add_token(&mut self, token_type: TokenType, text: &'a str) {
        self.tokens.push(Token { token_type, text });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let prev = self.current.clone();
        self.current += 1;
        self.source[prev]
    }

    fn advance_start(&mut self, num: usize) {
        self.start += num;
    }

    fn byte_at(&self, at: usize) -> u8 {
        self.source[at]
    }

    fn current_byte(&self) -> u8 {
        self.byte_at(self.current)
    }

    fn header(&mut self) {
        let mut num_of_hashtags = 1;
        while self.current_byte() == b'#' {
            self.advance();
            num_of_hashtags += 1;
        }

        while self.current_byte() != b'\n' && !self.is_at_end() {
            self.advance();
        }

        if num_of_hashtags <= 6 && self.byte_at(self.start + num_of_hashtags) == b' ' {
            self.advance_start(num_of_hashtags + 1);

            let mut end = self.current - 1;
            while self.source[end] == b'#' {
                end -= 1;
            }

            let bytes = &self.source[self.start..=end];

            let text = str::from_utf8(bytes).unwrap();

            self.add_token(TokenType::H(num_of_hashtags), text.trim());
        } else {
            let bytes = &self.source[self.start..self.current];
            let text = str::from_utf8(bytes).unwrap();
            self.add_token(TokenType::P, text);
        }
    }
}

#[test]
fn header() {
    let source = " 
        # header 1
        ## header 2
        ### header 3
        #### header 4
        ##### header 5
        ###### header 6
        
        ### this is a longer header
        # this is a header with a #hashtag in the middle
        
        # header 1 #
        ## header 2 ##
        ### header 3 ###
        #### header 4 ####
        ##### header 5 #####
        ###### header 6 ######
        ####### not a header
        #not a header
        ";

    let expected: Vec<Token> = vec![
    Token { text: "header 1", token_type: TokenType::H(1) },
    Token { text: "header 2", token_type: TokenType::H(2) },
    Token { text: "header 3", token_type: TokenType::H(3) },
    Token { text: "header 4", token_type: TokenType::H(4) },
    Token { text: "header 5", token_type: TokenType::H(5) },
    Token { text: "header 6", token_type: TokenType::H(6) },
    Token { text: "this is a longer header", token_type: TokenType::H(3) },
    Token { text: "this is a header with a #hashtag in the middle", token_type: TokenType::H(1) },
    Token { text: "header 1", token_type: TokenType::H(1) },
    Token { text: "header 2", token_type: TokenType::H(2) },
    Token { text: "header 3", token_type: TokenType::H(3) },
    Token { text: "header 4", token_type: TokenType::H(4) },
    Token { text: "header 5", token_type: TokenType::H(5) },
    Token { text: "header 6", token_type: TokenType::H(6) },
    Token { text: "####### not a header", token_type: TokenType::P },
    Token { text: "#not a header", token_type: TokenType::P },
    Token { text: "", token_type: TokenType::EOF },
    ];

    let mut lexer = Lexer::new(source);
    lexer.scan();

    for (i, e) in expected.iter().enumerate() {
        assert_eq!(e, &lexer.tokens[i], "failed at index {i}");
    }
}
