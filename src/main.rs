#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    /// Token Type
    pub kind: Kind,

    /// Start offset in source
    pub start: usize,

    /// End offset in source
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Eof, // end of file
    Plus,
}

fn main() {
    println!("Hello, world!");
}
