use std::{char, iter::Peekable, str::Chars};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Unknown,
    Eof,
    String(String),
    Identifier(String),
    Number(f32),
    True,               // true
    Maybe,              // maybe
    False,              // false
    If,                 // if
    Else,               // else
    ElseIf,             // elif
    For,                // for
    While,              // while
    Loop,               // loop
    Not,                // not
    And,                // and
    Or,                 // or 
    Of,                 // of
    End,                // end
    Break,              // break
    Class,              // class
    Function,           // func
    Public,             // pub
    Import,             // import
    LParen,             // (
    RParen,             // )
    LBrace,             // {
    RBrace,             // }
    LSqBrace,           // [
    RSqBrace,           // ]
    Dot,                // .
    Comma,              // ,
    Colon,              // :
    Add,                // +
    Sub,                // -
    Mul,                // *
    Div,                // /
    Pow,                // ^
    Assign,             // =
    AssignAdd,          // +=
    AssignSub,          // -=
    AssignDiv,          // /=
    AssignMul,          // *=
    AssignPow,          // ^=
    Equals,             // ==
    NotEquals,          // !=
    LessThan,           // <
    GreaterThan,        // >
    Returns,            // ->
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            chars: source.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(&ch) = self.chars.peek() {
            let mut tok: Token = Token::Unknown;
            let mut allow_next = true;

            match ch {
                // Parens
                '(' => tok = Token::LParen,
                ')' => tok = Token::RParen,

                // Braces
                '{' => tok = Token::LBrace,
                '}' => tok = Token::RBrace,

                // Square braces
                '[' => tok = Token::LSqBrace,
                ']' => tok = Token::RSqBrace,

                // Comparisons
                '<' => tok = Token::LessThan,
                '>' => tok = Token::GreaterThan,
                
                // Other
                ':' => tok = Token::Colon,
                ',' => tok = Token::Comma,
                '.' => tok = Token::Dot,

                // Advanced

                '=' | '!' | '+' | '-' | '*' | '/' | '^' => {
                    tok = self.read_multiple()
                }

                '\'' | '"' => {
                    self.chars.next();
                    tok = self.read_string(&ch);
                }
                
                _ => {
                    if is_skip(ch) {
                        self.chars.next();
                        continue;
                    }

                    if ch.is_alphabetic() {
                        tok = self.read_identifier();
                        allow_next = false;
                    } else if ch.is_numeric() {
                        tok = self.read_number();
                    }
                }
            }

            if allow_next {
                self.chars.next();
            }

            return tok;
        }
        return Token::Eof;
    }

    pub fn read_multiple(&mut self) -> Token {
        let mut string = String::new();

        while let Some(&ch) = self.chars.peek() {
            if ch.is_alphabetic() {
                break;
            }

            string.push(ch);
            self.chars.next();
        }

        match string.as_str() {
            "=" => Token::Assign,
            "+" => Token::Add,
            "-" => Token::Sub,
            "*" => Token::Mul,
            "/" => Token::Div,
            "^" => Token::Pow,
            
            "+=" => Token::AssignAdd,
            "-=" => Token::AssignSub,
            "*=" => Token::AssignMul,
            "^=" => Token::AssignPow,
            "/=" => Token::AssignDiv,
            "==" => Token::Equals,
            "!=" => Token::NotEquals,
            "->" => Token::Returns,
            
            _ => Token::Unknown
        }
    }

    pub fn read_string(&mut self, boundary: &char) -> Token {
        let mut string = String::new();

        while let Some(&ch) = self.chars.peek() {
            if ch.to_string() == boundary.to_string() {
                break;
            }

            string.push(ch);
            self.chars.next();
        }

        return Token::String(string);
    }

    pub fn read_number(&mut self) -> Token {
        let mut string = String::new();

        let mut frac = false;

        while let Some(&ch) = self.chars.peek() {
            if !ch.is_numeric() {
                if ch == '.' {
                    if frac {
                        return Token::Unknown;
                    } else {
                        frac = true;
                    }
                } else {
                    break;
                }
            }

            string.push(ch);
            self.chars.next();
        }

        return Token::Number(string.parse().unwrap());
    }

    pub fn read_identifier(&mut self) -> Token {
        let mut string = String::new();

        while let Some(&ch) = self.chars.peek() {
            if !ch.is_alphabetic() && !ch.is_numeric() {
                break;
            }

            string.push(ch);
            self.chars.next();
        }

        match string.as_str() {
            "true"   => Token::True,
            "maybe"  => Token::Maybe,
            "false"  => Token::False,
            "if"     => Token::If,
            "else"   => Token::Else,
            "elif"   => Token::ElseIf,
            "for"    => Token::For,
            "while"  => Token::While,
            "loop"   => Token::Loop,
            "not"    => Token::Not,
            "and"    => Token::And,
            "or"     => Token::Or,
            "of"     => Token::Of,
            "end"    => Token::End,
            "break"  => Token::Break,
            "class"  => Token::Class,
            "func"   => Token::Function,
            "pub"    => Token::Public, 
            "import" => Token::Import,

            _ => Token::Identifier(string)
        }
    }
}

pub fn is_skip(ch: char) -> bool {
    return ch == ' ' || ch == '\t' || ch == '\n';
}

pub fn lex(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    let mut tokens: Vec<Token> = Vec::new();

    loop {
        let token = lexer.next_token();
        tokens.push(token.clone());

        if token == Token::Eof {
            break;
        }
    }

    return tokens;
}