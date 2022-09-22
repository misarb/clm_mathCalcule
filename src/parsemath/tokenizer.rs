use super::token::Token;
use std::iter::Iterator;
use std::iter::Peekable;
use std::str::Chars;
// Define a TOKENIZER STRUCT
pub struct tokenizer<'a> {
    expre: Peekable<Chars<'a>>,
}

//IMPLEMENT new functionality for th Tokinizer structe
impl<'a> tokenizer<'a> {
    pub fn new(new_expr: &'a str) -> self {
        tokenizer {
            expre: new_expr.Chars().Peekable(),
        }
    }
}

//implement nex methode
impl<'a> Iterator for tokenizer<'a> {

    type item = token;
    fn next(&mut self) ->Option<Token>{
        let next_char = self.expre.next();

    }
}
