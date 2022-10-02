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
    fn next(&mut self) -> Option<Token> {
        let next_char = self.expre.next();
        match next_char {
            //this for number slicing it can be 12.32 as example we need to iterat it and peek to each char
            Some('0'..='9') => {
                let mut number = next_char?.to_string();
                while Some(next_char) = next_char.peek() {
                    if next_char.is_numeric() || next_char == &'.' {
                        number.push(self.expre.next()?);
                    } else if next_char == &'(' {
                        return None;
                    } else {
                        break;
                    }
                }
                Some(Token::Num(number.parse::<f64>().unwrap()));
            } //for number slicing
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Sub),
            Some('*') => Some(Token::Mul),
            Some('/') => Some(Token::Dived),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            Some('^') => Some(Token::Crate),
            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}
