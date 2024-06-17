use std::{collections::HashMap, iter::Peekable, vec::IntoIter};

use crate::tokenizer::TokenType;

pub fn parse(tokens: Vec<TokenType>) -> Result<JsonType, String> {
    let mut tokens = tokens.into_iter().peekable();

    parse_value(&mut tokens)
}

fn parse_value(tokens: &mut Peekable<IntoIter<TokenType>>) -> Result<JsonType, String> {
    let token = tokens.peek().ok_or(String::from("Invalid token"))?;

    let consume = |json_type: JsonType, tokens: &mut Peekable<IntoIter<TokenType>>| {
        tokens.next();
        Ok(json_type)
    } ;

    match token {
        TokenType::CurlyBraceOpening => parse_object(tokens),
        TokenType::BracketOpening => parse_array(tokens),
        TokenType::String(s) => consume(JsonType::String(s.clone()), tokens),
        TokenType::Null => consume(JsonType::Null, tokens),
        TokenType::Number(n) => consume(JsonType::Number(*n), tokens),
        TokenType::True => consume(JsonType::Boolean(true), tokens),
        TokenType::False => consume(JsonType::Boolean(false), tokens),
        _ => Err(String::from("Error parsing token")),
    }
}

fn parse_object(tokens: &mut Peekable<IntoIter<TokenType>>) -> Result<JsonType, String> {
    let mut json_types : HashMap<String, JsonType> = HashMap::new();

    while let Some(t) = tokens.peek() {
        return match t {
            TokenType::CurlyBraceOpening => get_key(tokens),
            _ => Err(String::from("Error parsing object.")),
        }
    }

    Err(String::from("sexo"))
}

fn get_key(tokens: &mut Peekable<IntoIter<TokenType>>) -> Result<JsonType, String> {

    tokens.next();
    let t = tokens.next().ok_or("Expected '}'.")?;

    todo!()
}

///Better parser, allows one trailing comma.
fn parse_array(tokens: &mut Peekable<IntoIter<TokenType>>) -> Result<JsonType, String> {
    let mut json_types = Vec::new();

    tokens.next();

    while let Some(t) = tokens.peek() {
        if *t == TokenType::BracketClosing {
            tokens.next();
            return Ok(JsonType::Array(json_types));
        }

        println!("{:?}", t);        

        json_types.push(parse_value(tokens)?);

        println!("sexo");

        let t = tokens.peek().ok_or("Expected ',' or ']'.")?;

        if *t == TokenType::Comma {
            tokens.next();
        }
    }

    Err(String::from("Error parsing array, ']' expected."))
}

#[derive(Debug, PartialEq)]
enum JsonType {
    Object(HashMap<String, JsonType>),
    Array(Vec<JsonType>),
    String(String),
    Number(f32),
    Boolean(bool),
    Null,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        parser::{parse, JsonType},
        tokenizer::TokenType,
    };

    #[test]
    fn try_parse() {
        let token_vec: Vec<TokenType> = vec![
            TokenType::BracketOpening,
            TokenType::Null,
            TokenType::Comma,
            TokenType::Number(15.0),
            TokenType::Comma,
            TokenType::True,
            TokenType::BracketClosing
        ];

        let a = parse(token_vec);
        let b = JsonType::Array(vec![
            JsonType::Null,
            JsonType::Number(15.0),
            JsonType::Boolean(true),
        ]);
        assert_eq!(a, Ok(b));
    }

    
    #[test]
    fn try_parse_with_trailing_comma() {
        let token_vec: Vec<TokenType> = vec![
            TokenType::BracketOpening,
            TokenType::Null,
            TokenType::Comma,
            TokenType::BracketClosing
        ];

        let a = parse(token_vec);
        let b = JsonType::Array(vec![
            JsonType::Null,
        ]);
        assert_eq!(a, Ok(b));
    }
}
