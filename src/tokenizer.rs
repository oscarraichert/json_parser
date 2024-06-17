use std::{iter::Peekable, str::Chars};


pub fn tokenize(string: &str) -> Result<Vec<TokenType>, String> {
    let mut chars = string.chars().peekable();
    let mut tokens = Vec::new();

    let consume = |token_type: TokenType, chars: &mut Peekable<Chars>| {
        chars.next();
        token_type
    };

    while let Some(char) = chars.peek() {
        let token = match char {
            '{' => consume(TokenType::CurlyBraceOpening, &mut chars),
            '}' => consume(TokenType::CurlyBraceClosing, &mut chars),
            '[' => consume(TokenType::BracketOpening, &mut chars),
            ']' => consume(TokenType::BracketClosing, &mut chars),
            ':' => consume(TokenType::Colon, &mut chars),
            ',' => consume(TokenType::Comma, &mut chars),
            'n' => parse_null(&mut chars)?,
            '"' => parse_string(&mut chars)?,
            't' => parse_true(&mut chars)?,
            'f' => parse_false(&mut chars)?,
            ' ' => {
                chars.next();
                continue;
            }
            char if char.is_digit(10) => parse_number(&mut chars)?,
            _ => return Err(String::from("Invalid character.")),
        };

        tokens.push(token);
    }

    Ok(tokens)
}

fn parse_number(chars: &mut Peekable<Chars>) -> Result<TokenType, String> {
    let mut number = String::new();

    while let Some(c) = chars.next() {
        if c.is_digit(10) || c == '.' {
            number.push(c);
        }
    }
    Ok(TokenType::Number(
        number.parse::<f32>().expect("Error parsing number"),
    ))
}

fn parse_string(chars: &mut Peekable<Chars>) -> Result<TokenType, String> {
    let mut string = String::new();

    chars.next();
    while let Some(c) = chars.next() {
        if c == '"' {
            return Ok(TokenType::String(string));
        }
        string.push(c);
    }

    Err(String::from("Error parsing string"))
}

fn parse_null(chars: &mut Peekable<Chars>) -> Result<TokenType, String> {
    if chars.next() != Some('n') {
        return Err(String::from("Error parsing null."));
    }

    if chars.next() != Some('u') {
        return Err(String::from("Error parsing null."));
    }

    if chars.next() != Some('l') {
        return Err(String::from("Error parsing null."));
    }

    if chars.next() != Some('l') {
        return Err(String::from("Error parsing null."));
    }

    Ok(TokenType::Null)
}

fn parse_true(chars: &mut Peekable<Chars>) -> Result<TokenType, String> {
    if chars.next() != Some('t') {
        return Err(String::from("Error parsing value."));
    }

    if chars.next() != Some('r') {
        return Err(String::from("Error parsing value."));
    }

    if chars.next() != Some('u') {
        return Err(String::from("Error parsing value."));
    }

    if chars.next() != Some('e') {
        return Err(String::from("Error parsing value."));
    }

    Ok(TokenType::True)
}

fn parse_false(chars: &mut Peekable<Chars>) -> Result<TokenType, String> {
    if chars.next() != Some('f') {
        return Err(String::from("Error parsing value."));
    }

    if chars.next() != Some('a') {
        return Err(String::from("Error parsing value."));
    }

    if chars.next() != Some('l') {
        return Err(String::from("Error parsing value."));
    }

    if chars.next() != Some('s') {
        return Err(String::from("Error parsing value."));
    }

    if chars.next() != Some('e') {
        return Err(String::from("Error parsing value."));
    }

    Ok(TokenType::False)
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    CurlyBraceOpening,
    CurlyBraceClosing,
    BracketOpening,
    BracketClosing,
    String(String),
    Null,
    Number(f32),
    Colon,
    Comma,
    True,
    False,
}

#[cfg(test)]
mod tests {
    use super::{tokenize, TokenType};

    #[test]
    fn try_parse() {
        let token_vec: Vec<TokenType> = vec![
            TokenType::CurlyBraceOpening,
            TokenType::String(String::from("id")),
            TokenType::Colon,
            TokenType::Number(15.0),
            TokenType::Comma,
            TokenType::String(String::from("value")),
            TokenType::Colon,
            TokenType::String(String::from("some value")),
            TokenType::CurlyBraceClosing,
        ];

        const JSON: &str = "{
            \"id\": 15,
            \"value\": \"some value\"
        }";

        let a = tokenize(JSON);
        assert_eq!(a, Ok(token_vec));
    }

    #[test]
    fn try_parse_null() {
        let tokens: Vec<TokenType> = vec![TokenType::Null];

        let json = "null";

        let a = tokenize(json);
        assert_eq!(a, Ok(tokens))
    }

    #[test]
    fn try_parse_true() {
        let tokens: Vec<TokenType> = vec![TokenType::True];

        let json = "true";

        let a = tokenize(json);
        assert_eq!(a, Ok(tokens))
    }

    #[test]
    fn try_parse_false() {
        let tokens: Vec<TokenType> = vec![TokenType::False];

        let json = "false";

        let a = tokenize(json);
        assert_eq!(a, Ok(tokens))
    }

    #[test]
    fn try_parse_string() {
        let tokens: Vec<TokenType> = vec![TokenType::String(String::from("teste"))];

        let json = "\"teste\"";

        let a = tokenize(json);
        assert_eq!(a, Ok(tokens))
    }

    #[test]
    fn try_parse_number() {
        let tokens: Vec<TokenType> = vec![TokenType::Number(15.0)];

        let json = "15";

        let a = tokenize(json);
        assert_eq!(a, Ok(tokens))
    }
}