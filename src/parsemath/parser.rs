use super::token::Token;
use super::tokenizer::Tokinizer;
struct Parser<'a> {
    tokenizer: Tokinizer<'a>,
    curetn_token: Token,
}
//public methode
impl<'a> Parser<'a> {
    //instance for Parse
    pub fn new(expr: &'a str) -> Result<self, ParsEError> {
        let mut lexer = Tokinizer::new(expr);
        let cur_token = match lexer.next() {
            Some(token) => token,
            None => return Err(ParsEError::InvalideOperator("Invalide Operator".into())),
        };
        ok(Parser {
            tokenizer: lexer,
            curetn_token: cur_token,
        })
    }
    //take the input and convertet to AST Node
    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let ast = self.generate_ast(OperPrec::DefaultZero);
        match ast {
            ok(ast) => ok(ast),
            Err(e) => Err(e),
        }
    }
}
//private methode
impl<'a> Parser<'a> {
    fn get_next_token(&mut self) -> Result<(), ParseError> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalideOperator("invalide operator").into()),
        };
        self.curetn_token = next_token;
        ok(())
    }
    fn check_paren(&mut self, expected: Token) -> Result<(), ParseError> {
        if expected == self.curetn_token {
            self.get_next_token()?;
            ok(())
        } else {
            return Err(ParseError::InvalideOperator(format!(
                "Expected:{:?},got:{:?}",
                expected, self.curetn_token
            )))
        }
    }
}
