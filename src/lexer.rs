use crate::error::BlazeError;
use crate::span::Span;
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Lexer {
    pub filename: String,
    pub source: String,
    pub tokens: Vec<Token>,
    pub current: usize,
    pub start: usize,
    pub end: usize,
}

impl Lexer {
    pub fn new(filename: String, source: String) -> Self {
        Self {
            filename,
            source,
            tokens: Vec::new(),
            current: 0,
            start: 0,
            end: 0,
        }
    }
    pub fn lex(&mut self) -> Result<Vec<Token>, Vec<BlazeError>> {
        let mut errors: Vec<BlazeError> = Vec::new();
        while self.current < self.source.len() {
            match self.current().clone() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                }
                '\n' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = start;
                    self.tokens.push(Token {
                        kind: TokenKind::Newline,
                        literal: None,
                        span: Span {
                            filename: self.filename.clone(),
                            start,
                            end: self.end,
                        },
                    })
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut literal: String = String::new();
                    let start: usize = self.start;
                    literal.push(self.current());
                    self.start += 1;
                    self.advance();
                    while self.current().is_alphanumeric() || self.current() == '_' {
                        literal.push(self.current());
                        self.start += 1;
                        self.advance();
                    }
                    self.end = self.start;
                    let kind: TokenKind = match literal.clone().as_str() {
                        "i8" => TokenKind::I8,
                        "i16" => TokenKind::I16,
                        "i32" => TokenKind::I32,
                        "i64" => TokenKind::I64,
                        "u8" => TokenKind::U8,
                        "u16" => TokenKind::U16,
                        "u32" => TokenKind::U32,
                        "u64" => TokenKind::U64,
                        "f32" => TokenKind::F32,
                        "f64" => TokenKind::F64,
                        "char" => TokenKind::Char,
                        "bool" => TokenKind::Bool,
                        "void" => TokenKind::Void,
                        "type" => TokenKind::Type,
                        "namespace" => TokenKind::Namespace,
                        "fn" => TokenKind::Fn,
                        "return" => TokenKind::Return,
                        "enum" => TokenKind::Enum,
                        "union" => TokenKind::Union,
                        "struct" => TokenKind::Struct,
                        "self" => TokenKind::SelfKeyword,
                        "while" => TokenKind::While,
                        "mut" => TokenKind::Mut,
                        "if" => TokenKind::If,
                        "else" => TokenKind::Else,
                        "import" => TokenKind::Import,
                        "comptime" => TokenKind::Comptime,
                        "try" => TokenKind::Try,
                        "null" => TokenKind::Null,
                        _ => TokenKind::Identifier,
                    };
                    self.tokens.push(Token {
                        kind,
                        literal: Some(literal),
                        span: Span {
                            filename: self.filename.clone(),
                            start,
                            end: self.end,
                        },
                    })
                }
                '0'..='9' => {
                    let mut literal: String = String::new();
                    let start: usize = self.start;
                    let mut is_float: bool = false;
                    let mut is_hex: bool = false;
                    let mut is_bin: bool = false;
                    if self.current() == '0' {
                        self.advance();
                        if self.current() == 'x' {
                            is_hex = true;
                            self.advance();
                            while self.current().is_numeric()
                                || self.current() == 'a'
                                || self.current() == 'b'
                                || self.current() == 'c'
                                || self.current() == 'd'
                                || self.current() == 'e'
                                || self.current() == 'f'
                            {
                                literal.push(self.current());
                                self.start += 1;
                                self.advance();
                            }
                        } else if self.current() == 'b' {
                            is_bin = true;
                            self.advance();
                            while self.current() == '0' || self.current() == '1' {
                                literal.push(self.current());
                                self.start += 1;
                                self.advance();
                            }
                        } else {
                            literal.push('0');
                            while self.current().is_numeric() {
                                literal.push(self.current());
                                self.start += 1;
                                self.advance();
                            }
                        }
                    } else {
                        while self.current().is_numeric() {
                            literal.push(self.current());
                            self.start += 1;
                            self.advance();
                        }
                    }
                    if self.current() == '.' {
                        is_float = true;
                        literal.push(self.current());
                        self.start += 1;
                        self.advance();
                        while self.current().is_numeric() {
                            literal.push(self.current());
                            self.start += 1;
                            self.advance();
                        }
                    }
                    self.end = self.start;
                    let kind: TokenKind = if is_float {
                        TokenKind::FloatLiteral
                    } else if is_hex {
                        TokenKind::HexadecimalLiteral
                    } else if is_bin {
                        TokenKind::BinaryLiteral
                    } else {
                        TokenKind::IntegerLiteral
                    };
                    self.tokens.push(Token {
                        kind,
                        literal: Some(literal),
                        span: Span {
                            filename: self.filename.clone(),
                            start,
                            end: self.end,
                        },
                    })
                }
                '"' => {
                    let mut literal: String = String::new();
                    self.advance();
                    self.start += 1;
                    let start: usize = self.start;
                    while self.current() != '"' {
                        literal.push(self.current());
                        if self.current() == '\\' {
                            self.start += 1;
                            self.advance();
                            literal.push(self.current());
                        }
                        self.start += 1;
                        self.advance();
                    }
                    self.end = self.start;
                    self.advance();
                    self.start += 1;
                    self.tokens.push(Token {
                        kind: TokenKind::StringLiteral,
                        literal: Some(literal),
                        span: Span {
                            filename: self.filename.clone(),
                            start,
                            end: self.end,
                        },
                    })
                }
                '\'' => {
                    let mut literal: String = String::new();
                    self.advance();
                    self.start += 1;
                    let start: usize = self.start;
                    if self.current() == '\\' {
                        self.start += 1;
                        self.advance();
                        match self.current() {
                            'n' => literal.push('\n'),
                            'r' => literal.push('\r'),
                            't' => literal.push('\t'),
                            '\\' => literal.push('\\'),
                            '\'' => literal.push('\''),
                            '"' => literal.push('"'),
                            '0' => literal.push('\0'),
                            _ => {
                                errors.push(BlazeError::SyntaxError(
                                    format!("invalid escape sequence: '\\{}'", self.current()),
                                    Span {
                                        filename: self.filename.clone(),
                                        start: self.start - 1,
                                        end: self.end,
                                    },
                                ));
                            }
                        }
                    } else {
                        literal.push(self.current());
                    }
                    self.start += 1;
                    self.advance();
                    self.end = self.start;
                    self.advance();
                    self.start += 1;
                    self.tokens.push(Token {
                        kind: TokenKind::CharLiteral,
                        literal: Some(literal),
                        span: Span {
                            filename: self.filename.clone(),
                            start,
                            end: self.end,
                        },
                    })
                }
                '(' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token {
                        kind: TokenKind::OpenParenthesis,
                        literal: None,
                        span: Span {
                            filename: self.filename.clone(),
                            start,
                            end: self.end,
                        },
                    })
                }
                ')' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token {
                        kind: TokenKind::CloseParenthesis,
                        literal: None,
                        span: Span {
                            filename: self.filename.clone(),
                            start,
                            end: self.end,
                        },
                    })
                }
                '[' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token {
                        kind: TokenKind::OpenBracket,
                        literal: None,
                        span: Span {
                            filename: self.filename.clone(),
                            start,
                            end: self.end,
                        },
                    })
                }
                ']' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token {
                        kind: TokenKind::CloseBracket,
                        literal: None,
                        span: Span {
                            filename: self.filename.clone(),
                            start,
                            end: self.end,
                        },
                    })
                }
                '{' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token {
                        kind: TokenKind::OpenBrace,
                        literal: None,
                        span: Span {
                            filename: self.filename.clone(),
                            start,
                            end: self.end,
                        },
                    })
                }
                '}' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token {
                        kind: TokenKind::CloseBrace,
                        literal: None,
                        span: Span {
                            filename: self.filename.clone(),
                            start,
                            end: self.end,
                        },
                    })
                }
                ':' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    if self.current() == ':' {
                        self.advance();
                        self.start += 1;
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::DoubleColon,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    } else if self.current() == '=' {
                        self.advance();
                        self.start += 1;
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::ColonEquals,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    } else {
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::Colon,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    }
                }
                ';' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token {
                        kind: TokenKind::Semicolon,
                        literal: None,
                        span: Span {
                            filename: self.filename.clone(),
                            start,
                            end: self.end,
                        },
                    })
                }
                '.' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    if self.current() == '.' {
                        self.advance();
                        self.start += 1;
                        if self.current() == '.' {
                            self.advance();
                            self.start += 1;
                            self.end = self.start;
                            self.tokens.push(Token {
                                kind: TokenKind::Elipsis,
                                literal: None,
                                span: Span {
                                    filename: self.filename.clone(),
                                    start,
                                    end: self.end,
                                },
                            })
                        } else {
                            self.end = self.start;
                            self.tokens.push(Token {
                                kind: TokenKind::DoubleDot,
                                literal: None,
                                span: Span {
                                    filename: self.filename.clone(),
                                    start,
                                    end: self.end,
                                },
                            })
                        }
                    } else {
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::Dot,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    }
                }
                ',' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token {
                        kind: TokenKind::Comma,
                        literal: None,
                        span: Span {
                            filename: self.filename.clone(),
                            start,
                            end: self.end,
                        },
                    })
                }
                '=' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    if self.current() == '=' {
                        self.advance();
                        self.start += 1;
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::EqualEqual,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    } else {
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::Equal,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    }
                }
                '-' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    if self.current() == '>' {
                        self.advance();
                        self.start += 1;
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::Arrow,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    } else if self.current() == '=' {
                        self.advance();
                        self.start += 1;
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::MinusEquals,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    } else {
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::Minus,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    }
                }
                '?' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token {
                        kind: TokenKind::QuestionMark,
                        literal: None,
                        span: Span {
                            filename: self.filename.clone(),
                            start,
                            end: self.end,
                        },
                    })
                }
                '!' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    if self.current() == '=' {
                        self.advance();
                        self.start += 1;
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::BangEqual,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    } else {
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::Bang,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    }
                }
                '>' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    if self.current() == '=' {
                        self.advance();
                        self.start += 1;
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::GreaterEqual,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    } else {
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::Greater,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    }
                }
                '<' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    if self.current() == '=' {
                        self.advance();
                        self.start += 1;
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::LessEqual,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    } else {
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::Less,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    }
                }
                '&' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token {
                        kind: TokenKind::Ampersand,
                        literal: None,
                        span: Span {
                            filename: self.filename.clone(),
                            start,
                            end: self.end,
                        },
                    })
                }
                '$' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token {
                        kind: TokenKind::Dollar,
                        literal: None,
                        span: Span {
                            filename: self.filename.clone(),
                            start,
                            end: self.end,
                        },
                    })
                }
                '+' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    if self.current() == '=' {
                        self.advance();
                        self.start += 1;
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::PlusEquals,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    } else {
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::Plus,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    }
                }
                '*' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    if self.current() == '=' {
                        self.advance();
                        self.start += 1;
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::AsteriskEquals,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    } else {
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::Asterisk,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    }
                }
                '/' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    if self.current() == '=' {
                        self.advance();
                        self.start += 1;
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::SlashEquals,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    } else {
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::Slash,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    }
                }
                '%' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    if self.current() == '=' {
                        self.advance();
                        self.start += 1;
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::PercentEquals,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    } else {
                        self.end = self.start;
                        self.tokens.push(Token {
                            kind: TokenKind::Percent,
                            literal: None,
                            span: Span {
                                filename: self.filename.clone(),
                                start,
                                end: self.end,
                            },
                        })
                    }
                }
                _ => {
                    let c: char = self.current();
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    errors.push(BlazeError::SyntaxError(
                        format!("Unexpected character: '{}'", c),
                        Span {
                            filename: self.filename.clone(),
                            start: self.start - 1,
                            end: self.end,
                        },
                    ));
                }
            }
        }
        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(self.tokens.clone())
        }
    }
    fn current(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }
}
