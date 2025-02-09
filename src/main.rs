use std::fs;

#[derive(Debug)]
enum Token {
    Global,
    Fn,
    Identifier(String),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Type(String),
    Comma,
}

#[derive(Debug)]
struct Ast {
    function_name: String,
    parameters: Vec<(String, String)>, // (type, name)
    is_global: bool,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\n' | '\t' | '\r' => {
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            '{' => {
                tokens.push(Token::LeftBrace);
                chars.next();
            }
            '}' => {
                tokens.push(Token::RightBrace);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            _ if c.is_alphabetic() => {
                let mut identifier = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        identifier.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                match identifier.as_str() {
                    "global" => tokens.push(Token::Global),
                    "fn" => tokens.push(Token::Fn),
                    s if s.starts_with("uint") => tokens.push(Token::Type(s.to_string())),
                    _ => tokens.push(Token::Identifier(identifier)),
                }
            }
            _ => {
                println!("Unexpected character: {}", c);
                chars.next();
            }
        }
    }
    tokens
}

fn parse(tokens: Vec<Token>) -> Option<Ast> {
    let mut iter = tokens.into_iter();
    let mut is_global = false;

    if let Some(Token::Global) = iter.next() {
        is_global = true;
    }

    match iter.next() {
        Some(Token::Fn) => (),
        _ => return None,
    }

    let function_name = match iter.next() {
        Some(Token::Identifier(name)) => name,
        _ => return None,
    };

    match iter.next() {
        Some(Token::LeftParen) => (),
        _ => return None,
    }

    let mut parameters = Vec::new();
    
    while let Some(token) = iter.next() {
        match token {
            Token::RightParen => break,
            Token::Type(typ) => {
                if let Some(Token::Identifier(name)) = iter.next() {
                    parameters.push((typ, name));
                }
            }
            _ => return None,
        }
    }

    Some(Ast {
        function_name,
        parameters,
        is_global,
    })
}

fn main() {
    let source = fs::read_to_string("src/hello.almiraj").expect("Failed to read file");
    let tokens = tokenize(&source);
    println!("Tokens: {:?}", tokens);
    
    if let Some(ast) = parse(tokens) {
        println!("AST: {:?}", ast);
    } else {
        println!("Failed to parse");
    }
}
