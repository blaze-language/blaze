use crate::span::Span;

#[derive(Debug, Clone)] pub struct Token {
    pub kind: TokenKind,
    pub literal: Option<String>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)] pub enum TokenKind {
    // Keywords
    Namespace,
    Fn,
    Return,
    Enum,
    Union,
    Struct,
    SelfKeyword,
    While,
    Mut,
    If,
    Else,
    Import,
    Comptime,

    // Types
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Char,
    Bool,
    Void,

    // Literals
    Identifier,
    StringLiteral,
    CharLiteral,
    IntegerLiteral,
    FloatLiteral,
    HexadecimalLiteral,
    BinaryLiteral,
    OctalLiteral,

    // Operators
    Equal,
    EqualEqual,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Punctuation
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Comma,
    Dot,
    DoubleDot,
    Elipsis, // ...
    Colon,
    DoubleColon,
    Semicolon,
    Arrow,
    QuestionMark,
    Bang,
    Ampersand,
    Dollar,

    // Arithmetic
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,

    // Assignment
    ColonEquals,
    PlusEquals,
    MinusEquals,
    AsteriskEquals,
    SlashEquals,
    PercentEquals,

    // Misc
    Newline,
    EndOfFile,
}