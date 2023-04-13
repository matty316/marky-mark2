#[derive(PartialEq, Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub text: &'a str,
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    H(usize), P, ULItem,

    EOF
}
