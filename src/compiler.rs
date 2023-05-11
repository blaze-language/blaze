use crate::ast::Statement;
use crate::lexer::Lexer;
use crate::error::BlazeError;
use crate::token::Token;
use crate::parser::Parser;

pub struct Compiler {
    pub files: Vec<String>,
    pub errors: Vec<BlazeError>,
}

impl Compiler { 
    pub fn new() -> Compiler {
        Compiler {
            files: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn compile(&mut self) -> Result<(), BlazeError> {
        for file in &self.files {
            println!("Compiling {}", file);

            let contents: Result<String, std::io::Error> = std::fs::read_to_string(file);
            if let Err(e) = contents {
                let error = BlazeError::IOError(std::rc::Rc::new(e));
                return Err(error);
            }

            let mut lexer: Lexer = Lexer::new(file.to_string(), contents.unwrap());
            let tokens: Result<Vec<Token>, Vec<BlazeError>> = lexer.lex();
            if let Err(errors) = tokens.clone() {
                self.errors.extend(errors);
                continue;
            }

            let mut parser: Parser = Parser::new(tokens.unwrap());
            let statements: Result<Vec<Statement>, Vec<BlazeError>> = parser.parse();
            if let Err(errors) = statements.clone() {
                self.errors.extend(errors);
                continue;
            }

            println!("Compiled {}", file);
        }

        Ok(())
    }

    pub fn add_file(&mut self, filename: String) {
        self.files.push(filename);
    }
}