use core::str;
use std::str::Chars;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// Token Type
    pub kind: Kind,

    /// Start offset in source
    pub start: usize,

    /// End offset in source
    pub end: usize,

    pub value: TokenValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    None,
    Number(f64),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Eof, // end of file
    Plus,
    Minus,
    Identifier,
    Number,
    String,
    If,
    While,
    For,
}

struct Lexer<'a> {
    source: &'a str,

    chars: Chars<'a>,
}

impl<'a> Lexer<'a>  {
    pub fn new(source: &'a str) -> Self {
        Self{
            source,
            chars: source.chars(),
        }
    }

    fn read_next_kind(&mut self) -> Kind {
        while let Some(c) = self.chars.next() {
            println!("read kind: {}", c);
            match c {
                '+' => return Kind::Plus,
                '-' => return Kind::Minus,
                _ => {}
            }
        }
        Kind::Eof
    }

    fn read_next_token(&mut self) -> Token {
        let start = self.offset();
        let kind = self.read_next_kind();
        let end = self.offset();
        Token{ kind, start, end, value: TokenValue::String("token".to_string()) }
    }

    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn match_keyward(&self, ident: &str) -> Kind {
        if ident.len() == 1 || ident.len() > 10 {
            return Kind::Identifier;
        }
        match ident {
            "if" => Kind::If,
            "while" => Kind::While,
            "for" => Kind::For,
            _ => Kind::Identifier,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct Node {
    /// ソース内の開始オフセット
    pub start: usize,

    /// ソース内の終了オフセット
    pub end: usize,
}

impl Node {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

pub struct Program {
    pub node: Node,
    pub body: Vec<Statement>,
}

pub enum Statement {
    VariableDeclarationStatement(VariableDeclaration),
}

pub struct VariableDeclaration {
    pub node: Node,
    pub id: BindingIdentifier,
    pub init: Option<Expression>,
}

pub struct BindingIdentifier {
    pub node: Node,
    pub name: String,
}

pub enum Expression {
}

pub struct AwaitExpression {
    pub node: Node,
    pub expression: Box<Expression>,
}

pub struct YieldExpression {
    pub node: Node,
    pub expression: Box<Expression>,
}

fn main() {
    let mut l = Lexer::new("+-");
    println!("offset: {}", l.offset());
    println!("{:?}", l.read_next_kind());
    println!("{:?}", l.read_next_kind());
    println!("{:?}", l.read_next_token());
    println!("{:?}", l.peek());
    // println!("{:?}", l.match_keyward("if"));
    // println!("{:?}", l.match_keyward("for"));
    // println!("{:?}", l.match_keyward("while"));
    // println!("{:?}", l.match_keyward("whilewhilewhilewhile"));
}
