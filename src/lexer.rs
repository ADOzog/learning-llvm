/*
  The Lexer
*/
#[derive(Debug, PartialEq)]
pub enum Token {
    //End of file
    Eof,

    //Commands
    Def,
    Extern,
    // Vars
    Identifier(String),
    Number(f64),
    Char(char),

    // Control flow, Not used yet?
    If,
    Then,
    Else,

    //
    For,
    In,
}

#[derive(Debug, PartialEq)]
enum LexerModes {
    Alpha,
    Num,
    Comment,
    Char,
    WhiteSpace,
}

fn match_to_keyword_or_ident(input_string: &String) -> Token {
    match input_string.as_ref() {
        "def" => Token::Def,
        "extern" => Token::Extern,
        "if" => Token::If,
        "then" => Token::Then,
        "else" => Token::Else,
        "for" => Token::For,
        "in" => Token::In,
        _ => Token::Identifier(input_string.clone()),
    }
}

// TBH if rust had a tail-call I would make this tail recursive
pub fn gettok(raw_text: &str) -> Vec<Token> {
    let mut token_vec: Vec<Token> = Vec::new();

    let mut alpha_string: String = String::new();
    let mut num_string: String = String::new();
    let mut prev = LexerModes::WhiteSpace;

    let text_iter = raw_text.chars();
    for c in text_iter {
        match (c, &prev) {
            (c, LexerModes::WhiteSpace) if c.is_ascii_whitespace() => (),
            (c, LexerModes::WhiteSpace) if c.is_ascii_alphabetic() => {
                alpha_string.push(c);
                prev = LexerModes::Alpha;
            }
            (c, LexerModes::WhiteSpace) if c.is_ascii_digit() => {
                num_string.push(c);
                prev = LexerModes::Num;
            }
            (c, LexerModes::Alpha) if c.is_ascii_alphabetic() => {
                alpha_string.push(c);
            }
            (c, LexerModes::Num) if c.is_ascii_digit() || c == '.' => {
                num_string.push(c);
            }
            (c, LexerModes::Alpha) if c.is_ascii_digit() => {
                alpha_string.push(c);
            }
            (c, LexerModes::Num) if c.is_ascii_alphabetic() => panic!(),
            (c, LexerModes::Num) if c.is_ascii_whitespace() => {
                let num: f64 = num_string.parse().unwrap_or_default();
                token_vec.push(Token::Number(num));
                num_string = String::new();
            }
            (c, LexerModes::Alpha) if c.is_ascii_whitespace() => {
                token_vec.push(match_to_keyword_or_ident(&alpha_string));
                alpha_string = String::new();
                prev = LexerModes::WhiteSpace
            }
            (c, LexerModes::Comment) if c == '\r' || c == '\n' => prev = LexerModes::WhiteSpace,
            (c, _) => {
                if c == '#' {
                    prev = LexerModes::Comment;
                    continue;
                }
                if prev != LexerModes::Comment {
                    if alpha_string != String::new() {
                        token_vec.push(match_to_keyword_or_ident(&alpha_string));
                        alpha_string = String::new()
                    }
                    if num_string != String::new() {
                        let num: f64 = num_string.parse().unwrap_or_default();
                        token_vec.push(Token::Number(num));
                        num_string = String::new()
                    }
                    token_vec.push(Token::Char(c.into()))
                }
            }
        }
    }
    if alpha_string != String::new() {
        token_vec.push(match_to_keyword_or_ident(&alpha_string));
    }
    if num_string != String::new() {
        let num: f64 = num_string.parse().unwrap_or_default();
        token_vec.push(Token::Number(num));
    }

    token_vec.push(Token::Eof);
    return token_vec;
}

#[cfg(test)]
mod test {
    use crate::lexer::{Token, gettok};

    #[test]
    fn test_identifier() {
        //println!("{:#?}", gettok("a b c"));
        assert_eq!(
            vec![
                Token::Identifier('a'.into()),
                Token::Identifier('b'.into()),
                Token::Identifier('c'.into()),
                Token::Eof
            ],
            gettok("a b c")
        );
    }

    #[test]
    fn test_keywords() {
        assert_eq!(
            vec![Token::Def, Token::Extern, Token::Eof],
            gettok("def extern")
        );
    }

    #[test]
    fn test_number() {
        assert_eq!(vec![Token::Number(12.34f64), Token::Eof], gettok("12.34"));
        assert_eq!(
            vec![
                Token::Number(1.0f64),
                Token::Number(2.0f64),
                Token::Number(3.0f64),
                Token::Eof
            ],
            gettok("1.0 2.0 3.0")
        );
        assert_eq!(vec![Token::Number(0f64), Token::Eof], gettok("12.12.12"));
    }

    #[test]
    fn test_comment() {
        assert_eq!(vec![Token::Eof], gettok("      # asdjhtasldk alsdkhjf;l "));
        assert_eq!(
            vec![
                Token::Identifier("abc".into()),
                Token::Number(3.14f64),
                Token::Eof
            ],
            gettok("abc # my comment to end \n 3.14")
        );
        assert_eq!(vec![Token::Def, Token::Eof], gettok("def#extern"));
    }

    #[test]
    fn test_chars() {
        assert_eq!(
            vec![
                Token::Identifier("a".into()),
                Token::Char('+'),
                Token::Identifier("b".into()),
                Token::Char('-'),
                Token::Identifier("c".into()),
                Token::Eof
            ],
            gettok("a+b-c")
        );
    }

    #[test]
    fn test_whitespaces() {
        assert_eq!(
            vec![
                Token::Char('+'),
                Token::Identifier("a".into()),
                Token::Identifier("b".into()),
                Token::Identifier("c".into()),
                Token::Char('!'),
                Token::Eof
            ],
            gettok("           +a   b      c!")
        );
    }

    #[test]
    fn test_control_flow() {
        assert_eq!(
            vec![Token::If, Token::Then, Token::Else, Token::Eof],
            gettok("  if then # \r else ")
        );
    }

    #[test]
    fn test_loops() {
        assert_eq!(
            vec![
                Token::For,
                Token::Identifier("Word".into()),
                Token::In,
                Token::Eof
            ],
            gettok("  for Word in ")
        )
    }
}
