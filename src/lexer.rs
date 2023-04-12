use crate::token::{Token, TokenType};

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

    pub fn scan(&self) {
       while !self.is_at_end() {

       } 
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
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

    let lexer = Lexer::new(source);
    lexer.scan();

    assert_eq!(expected, lexer.tokens);
}
