use crate::token::{Token, TokenKind};
use crate::error::BlazeError;
use crate::ast::{Statement, Expression, Type, StructField, EnumVariant};
use crate::span::Span;

#[derive(Debug, Clone)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub errors: Vec<BlazeError>,
    pub statements: Vec<Statement>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            errors: Vec::new(),
            statements: Vec::new(),
            current: 0,
        }
    }
    pub fn parse(&mut self) -> Result<Vec<Statement>, Vec<BlazeError>> {
        while self.current < self.tokens.len() {
            let statement: Result<Statement, BlazeError> = self.parse_statement();
            match statement {
                Ok(statement) => self.statements.push(statement),
                Err(error) => {
                    self.errors.push(error);
                    self.synchronize();
                }
            }
        }

        if self.errors.len() == 0 {
            Ok(self.statements.clone())
        } else {
            Err(self.errors.clone())
        }
    }
    fn synchronize(&mut self) {
        while self.current < self.tokens.len() {
            // if self.tokens[self.current].kind == TokenKind::Newline {
            //     self.current += 1;
            //     return;
            // }
            self.current += 1;
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, BlazeError> {
        match self.current()?.kind {
            TokenKind::Identifier => self.parse_identifier(),
            TokenKind::Return => self.parse_return(),
            TokenKind::Mut => self.parse_mut(),
            TokenKind::While => self.parse_while(),
            TokenKind::If => self.parse_if(),
            _ => {
                let span: Span = self.current()?.span;
                let expression: Expression = self.parse_expression()?;
                // self.expect(TokenKind::Newline)?;
                self.expect(TokenKind::Semicolon)?;
                Ok(Statement::Expression(expression, span))
            }
        }
    }

    fn parse_identifier(&mut self) -> Result<Statement, BlazeError> {
        let span: Span = self.current()?.span;
        if self.peek()?.kind == TokenKind::DoubleColon {
            let identifier: String = self.expect(TokenKind::Identifier)?.literal.unwrap();
            self.expect(TokenKind::DoubleColon)?;
            if self.current()?.kind == TokenKind::Namespace {
                self.parse_namespace(identifier, span)
            } else if self.current()?.kind == TokenKind::Struct {
                self.parse_struct(identifier, span)
            } else if self.current()?.kind == TokenKind::Enum {
                self.parse_enum(identifier, span)
            } else if self.current()?.kind == TokenKind::Union {
                self.parse_union(identifier, span)
            } else if self.current()?.kind == TokenKind::Fn {
                self.parse_fn(identifier, span)
            } else if self.current()?.kind == TokenKind::Import {
                self.parse_import(identifier, span)
            } else {
                let value: Expression = self.parse_expression()?;
                Ok(Statement::ConstantDeclaration(identifier, Type::AwaitingInference, value, span))
            }
        } else if self.peek()?.kind == TokenKind::Colon {
            let identifier: String = self.expect(TokenKind::Identifier)?.literal.unwrap();
            self.expect(TokenKind::Colon)?;
            let ty: Type = self.parse_type()?;
            self.expect(TokenKind::Equal)?;
            let value: Expression = self.parse_expression()?;
            self.expect(TokenKind::Semicolon)?;
            Ok(Statement::VariableDeclaration(identifier, ty, value, span))
        } else if self.peek()?.kind == TokenKind::ColonEquals {
            let identifier: String = self.expect(TokenKind::Identifier)?.literal.unwrap();
            self.expect(TokenKind::ColonEquals)?;
            let value: Expression = self.parse_expression()?;
            self.expect(TokenKind::Semicolon)?;
            Ok(Statement::VariableDeclaration(identifier, Type::AwaitingInference, value, span))
        } else {
            let expression: Expression = self.parse_expression()?;
            self.expect(TokenKind::Semicolon)?;
            Ok(Statement::Expression(expression, span))
        }
    }
    fn parse_namespace(&mut self, identifier: String, span: Span) -> Result<Statement, BlazeError> {
        self.expect(TokenKind::Namespace)?;
        let mut statements: Vec<Statement> = Vec::new();
        self.expect(TokenKind::OpenBrace)?;
        
        while self.current()?.kind != TokenKind::CloseBrace {
            let statement: Statement = self.parse_statement()?;
            statements.push(statement);
        }
        
        self.expect(TokenKind::CloseBrace)?;
        
        Ok(Statement::Namespace(identifier, statements, span))
    }
    fn parse_struct(&mut self, identifier: String, span: Span) -> Result<Statement, BlazeError> {
        self.expect(TokenKind::Struct)?;
        let mut generic_parameters: Vec<Type> = Vec::new();
        if self.current()?.kind == TokenKind::Less {
            self.expect(TokenKind::Less)?;
            generic_parameters.push(self.parse_type()?);
            while self.current()?.kind == TokenKind::Comma {
                self.expect(TokenKind::Comma)?;
                generic_parameters.push(self.parse_type()?);
            }
            self.expect(TokenKind::Greater)?;
        }
        let inherits: Vec<String> = Vec::new();
        let mut fields: Vec<StructField> = Vec::new();
        self.expect(TokenKind::OpenBrace)?;
        
        while self.current()?.kind != TokenKind::CloseBrace {
            let span: Span = self.current()?.span;
            let identifier: String = self.expect(TokenKind::Identifier)?.literal.unwrap();
            self.expect(TokenKind::Colon)?;
            let ty: Type = self.parse_type()?;
            fields.push(StructField {
                name: identifier,
                ty: ty,
                span: span,
            });
            if self.current()?.kind == TokenKind::Comma {
                self.expect(TokenKind::Comma)?;
            } else {
                break;
            }
        }
        
        self.expect(TokenKind::CloseBrace)?;
        
        Ok(Statement::Struct(identifier, generic_parameters, inherits, fields, span))
    }
    fn parse_enum(&mut self, identifier: String, span: Span) -> Result<Statement, BlazeError> {
        self.expect(TokenKind::Enum)?;
        let mut inner_ty: Option<Type> = None;
        if self.current()?.kind == TokenKind::OpenParenthesis {
            self.expect(TokenKind::OpenParenthesis)?;
            inner_ty = Some(self.parse_type()?);
            self.expect(TokenKind::CloseParenthesis)?;
        }
        let mut variants: Vec<EnumVariant> = Vec::new();
        self.expect(TokenKind::OpenBrace)?;
        
        while self.current()?.kind != TokenKind::CloseBrace {
            let span: Span = self.current()?.span;
            let identifier: String = self.expect(TokenKind::Identifier)?.literal.unwrap();
            if self.current()?.kind == TokenKind::Comma {
                self.expect(TokenKind::Comma)?;
                variants.push(EnumVariant::Unit(identifier, span));
                continue;
            } else if self.current()?.kind == TokenKind::CloseBrace {
                variants.push(EnumVariant::Unit(identifier, span));
                break;
            }
            if self.current()?.kind == TokenKind::Equal && inner_ty.is_none() {
                return Err(BlazeError::ParseError(format!("cannot assign value to enum variant without inner type"), span));
            }
            self.expect(TokenKind::Equal)?;
            let expression: Expression = self.parse_expression()?;
            variants.push(EnumVariant::Expression(identifier, expression, span));
            if self.current()?.kind == TokenKind::Comma {
                self.expect(TokenKind::Comma)?;
            } else {
                break;
            }
        }
        
        self.expect(TokenKind::CloseBrace)?;
        
        if inner_ty.is_some() {
            Ok(Statement::TypedEnum(identifier, inner_ty.unwrap(), variants, span))
        } else {
            Ok(Statement::Enum(identifier, variants, span))
        }
    }
    fn parse_union(&mut self, identifier: String, span: Span) -> Result<Statement, BlazeError> {
        self.expect(TokenKind::Union)?;
        let mut types: Vec<Type> = Vec::new();
        self.expect(TokenKind::OpenBrace)?;
        
        while self.current()?.kind != TokenKind::CloseBrace {
            let ty: Type = self.parse_type()?;
            if self.current()?.kind == TokenKind::Comma {
                self.expect(TokenKind::Comma)?;
                types.push(ty);
            } else {
                types.push(ty);
                break;
            }
        }
        
        self.expect(TokenKind::CloseBrace)?;
        
        Ok(Statement::Union(identifier, types, span))
    }
    fn parse_fn(&mut self, identifier: String, span: Span) -> Result<Statement, BlazeError> {
        self.expect(TokenKind::Fn)?;
        let mut parameters: Vec<(String, Type, bool, Span)> = Vec::new();
        let mut returns: Vec<Type> = vec![Type::Void(span.clone())];
        let mut struct_name: Option<Type> = None;
        self.expect(TokenKind::OpenParenthesis)?;
        
        while self.current()?.kind != TokenKind::CloseParenthesis {
            let span: Span = self.current()?.span;
            let comptime: bool = if self.current()?.kind == TokenKind::Comptime {
                self.expect(TokenKind::Comptime)?;
                true
            } else { false };
            if self.current()?.kind == TokenKind::SelfKeyword {
                self.expect(TokenKind::SelfKeyword)?;
                self.expect(TokenKind::Colon)?;
                let ty: Type = self.parse_type()?;
                struct_name = Some(ty);
                if self.current()?.kind != TokenKind::CloseParenthesis {
                    self.expect(TokenKind::Comma)?;
                }
                continue;
            }
            let identifier: String = self.expect(TokenKind::Identifier)?.literal.unwrap();
            self.expect(TokenKind::Colon)?;
            if self.current()?.kind == TokenKind::Elipsis {
                let var_args_span: Span = self.current()?.span;
                self.expect(TokenKind::Elipsis)?;
                parameters.push((identifier, Type::VarArgs(Box::new(None), var_args_span.clone()), comptime, span));
                break;
            }
            let ty: Type = self.parse_type()?;
            if self.current()?.kind == TokenKind::Elipsis {
                let var_args_span: Span = self.current()?.span;
                self.expect(TokenKind::Elipsis)?;
                parameters.push((identifier, Type::VarArgs(Box::new(Some(ty)), var_args_span.clone()), comptime, span));
                break;
            }
            if self.current()?.kind != TokenKind::CloseParenthesis {
                self.expect(TokenKind::Comma)?;
            }
            parameters.push((identifier, ty, comptime, span));
        }
        
        self.expect(TokenKind::CloseParenthesis)?;
        
        if self.current()?.kind == TokenKind::Arrow {
            self.expect(TokenKind::Arrow)?;
            returns.push(self.parse_type()?);
            while self.current()?.kind == TokenKind::Comma {
                self.expect(TokenKind::Comma)?;
                returns.push(self.parse_type()?);
            }
        }
        
        let mut statements: Vec<Statement> = Vec::new();
        self.expect(TokenKind::OpenBrace)?;
        
        while self.current()?.kind != TokenKind::CloseBrace {
            let statement: Statement = self.parse_statement()?;
            statements.push(statement);
        }
        
        self.expect(TokenKind::CloseBrace)?;
        
        if struct_name.is_some() {
            Ok(Statement::StructFunction(struct_name.unwrap(), identifier, parameters, returns, statements, span))
        } else {
            Ok(Statement::Function(identifier, parameters, returns, statements, span))
        }
    }
    fn parse_import(&mut self, identifier: String, span: Span) -> Result<Statement, BlazeError> {
        self.expect(TokenKind::Import)?;
        let path: String = self.expect(TokenKind::StringLiteral)?.literal.unwrap();
        
        Ok(Statement::Import(identifier, path, span))
    }
    fn parse_return(&mut self) -> Result<Statement, BlazeError> {
        let span: Span = self.current()?.span;
        self.expect(TokenKind::Return)?;
        if self.current()?.kind == TokenKind::Semicolon {
            self.expect(TokenKind::Semicolon)?;
            Ok(Statement::Return(vec![], span))
        } else {
            let mut expressions: Vec<Expression> = vec![];
            expressions.push(self.parse_expression()?);
            while self.current()?.kind == TokenKind::Comma {
                self.expect(TokenKind::Comma)?;
                expressions.push(self.parse_expression()?);
            }
            self.expect(TokenKind::Semicolon)?;
            Ok(Statement::Return(expressions, span))
        }
    }
    fn parse_mut(&mut self) -> Result<Statement, BlazeError> {
        self.expect(TokenKind::Mut)?;
        let span: Span = self.current()?.span;
        let identifier: String = self.expect(TokenKind::Identifier)?.literal.unwrap();
        let mut ty: Type = Type::AwaitingInference;
        if self.current()?.kind == TokenKind::ColonEquals {
            self.expect(TokenKind::ColonEquals)?;
        } else {
            self.expect(TokenKind::Colon)?;
            ty = self.parse_type()?;
            self.expect(TokenKind::Equal)?;
        }
        let value: Expression = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Statement::MutableDeclaration(identifier, ty, value, span))
    }
    fn parse_while(&mut self) -> Result<Statement, BlazeError> {
        let span: Span = self.current()?.span;
        self.expect(TokenKind::While)?;
        let expression: Expression = self.parse_expression()?;
        let mut statements: Vec<Statement> = Vec::new();
        self.expect(TokenKind::OpenBrace)?;
        
        while self.current()?.kind != TokenKind::CloseBrace {
            let statement: Statement = self.parse_statement()?;
            statements.push(statement);
        }
        
        self.expect(TokenKind::CloseBrace)?;
        
        Ok(Statement::While(expression, statements, span))
    }

    fn parse_if(&mut self) -> Result<Statement, BlazeError> {
        self.expect(TokenKind::If)?;
        let span: Span = self.current()?.span;
        let expression: Expression = self.parse_expression()?;
        let mut if_statements: Vec<Statement> = Vec::new();
        let mut else_statements: Vec<Statement> = Vec::new();
        self.expect(TokenKind::OpenBrace)?;
        
        while self.current()?.kind != TokenKind::CloseBrace {
            let statement: Statement = self.parse_statement()?;
            if_statements.push(statement);
        }
        
        self.expect(TokenKind::CloseBrace)?;
        
        if self.current()?.kind == TokenKind::Else {
            self.expect(TokenKind::Else)?;
            self.expect(TokenKind::OpenBrace)?;
            
            while self.current()?.kind != TokenKind::CloseBrace {
                let statement: Statement = self.parse_statement()?;
                else_statements.push(statement);
            }
            
            self.expect(TokenKind::CloseBrace)?;
            
        }
        Ok(Statement::If(expression, if_statements, else_statements, span))
    }

    fn parse_expression(&mut self) -> Result<Expression, BlazeError> {
        self.parse_member_access()
    }
    fn parse_member_access(&mut self) -> Result<Expression, BlazeError> {
        let span: Span = self.current()?.span;
        let mut expression: Expression = self.parse_binary_operation()?;
        while self.current()?.kind == TokenKind::Dot || self.current()?.kind == TokenKind::DoubleColon {
            let mut is_static: bool = false;
            if self.current()?.kind == TokenKind::Dot {
                self.expect(TokenKind::Dot)?;
            } else {
                self.expect(TokenKind::DoubleColon)?;
                is_static = true;
            }
            let member: Expression = self.parse_expression()?;
            if is_static {
                expression = Expression::StaticMemberAccess(Box::new(expression), Box::new(member), span.clone());
            } else {
                expression = Expression::MemberAccess(Box::new(expression), Box::new(member), span.clone());
            }
        }
        Ok(expression)
    }
    fn parse_binary_operation(&mut self) -> Result<Expression, BlazeError> {
        let span: Span = self.current()?.span;
        let mut expression: Expression = self.parse_primary()?;
        while self.current()?.kind == TokenKind::Plus
                || self.current()?.kind == TokenKind::Minus
                || self.current()?.kind == TokenKind::Asterisk
                || self.current()?.kind == TokenKind::Slash
                || self.current()?.kind == TokenKind::Percent
                || self.current()?.kind == TokenKind::EqualEqual
                || self.current()?.kind == TokenKind::BangEqual
                || self.current()?.kind == TokenKind::Greater
                || self.current()?.kind == TokenKind::GreaterEqual
                || self.current()?.kind == TokenKind::Less
                || self.current()?.kind == TokenKind::LessEqual
                || self.current()?.kind == TokenKind::PlusEquals
                || self.current()?.kind == TokenKind::MinusEquals
                || self.current()?.kind == TokenKind::AsteriskEquals
                || self.current()?.kind == TokenKind::SlashEquals
                || self.current()?.kind == TokenKind::PercentEquals {
            let operator: Token = self.current()?.clone();
            self.advance()?;
            let right: Expression = self.parse_expression()?;
            expression = Expression::BinaryOperation(Box::new(expression), operator.kind, Box::new(right), span.clone());
        }
        Ok(expression)
    }
    fn parse_primary(&mut self) -> Result<Expression, BlazeError> {
        let span = self.current()?.span;
        match self.current()?.kind {
            TokenKind::Identifier => {
                let identifier: String = self.expect(TokenKind::Identifier)?.literal.unwrap();
                if self.current()?.kind == TokenKind::OpenBracket {
                    self.expect(TokenKind::OpenBracket)?;
                    let index: Expression = self.parse_expression()?;
                    self.expect(TokenKind::CloseBracket)?;
                    Ok(Expression::ArrayAccess(identifier, Box::new(index), span))
                } else if self.current()?.kind == TokenKind::OpenBrace {
                    self.expect(TokenKind::OpenBrace)?;
                    let mut fields: Vec<(Option<String>, Expression, Span)> = Vec::new();
                    while self.current()?.kind != TokenKind::CloseBrace {
                        let span: Span = self.current()?.span;
                        let mut identifier: Option<String> = None;
                        if self.peek()?.kind == TokenKind::Colon {
                            identifier = Some(self.expect(TokenKind::Identifier)?.literal.unwrap());
                            self.expect(TokenKind::Colon)?;
                        }
                        let expression: Expression = self.parse_expression()?;
                        if self.current()?.kind == TokenKind::Comma {
                            self.expect(TokenKind::Comma)?;
                        } else {
                            break;
                        }
                        fields.push((identifier, expression, span));
                    }
                    
                    self.expect(TokenKind::CloseBrace)?;
                    Ok(Expression::StructLiteral(identifier, fields, span))
                } else {
                    Ok(Expression::Identifier(identifier, span))
                }
            }
            TokenKind::IntegerLiteral => {
                let integer: i64 = self.expect(TokenKind::IntegerLiteral)?.literal.unwrap().parse::<i64>().unwrap();
                Ok(Expression::Integer(integer, span))
            }
            TokenKind::CharLiteral => {
                let character: char = self.expect(TokenKind::CharLiteral)?.literal.unwrap().parse::<char>().unwrap();
                Ok(Expression::Char(character, span))
            }
            TokenKind::StringLiteral => {
                let string: String = self.expect(TokenKind::StringLiteral)?.literal.unwrap();
                Ok(Expression::String(string, span))
            }
            TokenKind::SelfKeyword => {
                self.expect(TokenKind::SelfKeyword)?;
                Ok(Expression::SelfLiteral(span))
            }
            TokenKind::Ampersand => {
                self.expect(TokenKind::Ampersand)?;
                let expression: Expression = self.parse_expression()?;
                Ok(Expression::AddressOf(Box::new(expression), span))
            }
            TokenKind::Asterisk => {
                self.expect(TokenKind::Asterisk)?;
                let expression: Expression = self.parse_expression()?;
                Ok(Expression::Dereference(Box::new(expression), span))
            }
            TokenKind::Try => {
                self.expect(TokenKind::Try)?;
                let expression: Expression = self.parse_expression()?;
                Ok(Expression::Try(Box::new(expression), span))
            }
            TokenKind::Null => {
                self.expect(TokenKind::Null)?;
                Ok(Expression::Null(span))
            }
            _ => {
                Err(BlazeError::ParseError(format!("expected expression, but got {:?}", self.current()?.kind), span))
            }
        }
    }

    fn parse_type(&mut self) -> Result<Type, BlazeError> {
        let span: Span = self.current()?.span;
        let t: Type = match self.current()?.kind.clone() {
            TokenKind::I8 => {
                self.expect(TokenKind::I8)?;
                Type::I8(span.clone())
            }
            TokenKind::I16 => {
                self.expect(TokenKind::I16)?;
                Type::I16(span.clone())
            }
            TokenKind::I32 => {
                self.expect(TokenKind::I32)?;
                Type::I32(span.clone())
            }
            TokenKind::I64 => {
                self.expect(TokenKind::I64)?;
                Type::I64(span.clone())
            }
            TokenKind::U8 => {
                self.expect(TokenKind::U8)?;
                Type::U8(span.clone())
            }
            TokenKind::U16 => {
                self.expect(TokenKind::U16)?;
                Type::U16(span.clone())
            }
            TokenKind::U32 => {
                self.expect(TokenKind::U32)?;
                Type::U32(span.clone())
            }
            TokenKind::U64 => {
                self.expect(TokenKind::U64)?;
                Type::U64(span.clone())
            }
            TokenKind::F32 => {
                self.expect(TokenKind::F32)?;
                Type::F32(span.clone())
            }
            TokenKind::F64 => {
                self.expect(TokenKind::F64)?;
                Type::F64(span.clone())
            }
            TokenKind::Bool => {
                self.expect(TokenKind::Bool)?;
                Type::Bool(span.clone())
            }
            TokenKind::Char => {
                self.expect(TokenKind::Char)?;
                Type::Char(span.clone())
            }
            TokenKind::Void => {
                self.expect(TokenKind::Void)?;
                Type::Void(span.clone())
            }
            TokenKind::Type => {
                self.expect(TokenKind::Type)?;
                Type::Type(span.clone())
            }
            TokenKind::Asterisk => {
                self.expect(TokenKind::Asterisk)?;
                let t: Type = self.parse_type()?;
                Type::Pointer(Box::new(t), span.clone())
            }
            TokenKind::OpenBracket => {
                self.expect(TokenKind::OpenBracket)?;
                self.expect(TokenKind::CloseBracket)?;
                let t: Type = self.parse_type()?;
                Type::Array(Box::new(t), span.clone())
            }
            TokenKind::QuestionMark => {
                self.expect(TokenKind::QuestionMark)?;
                let t: Type = self.parse_type()?;
                Type::Optional(Box::new(t), span.clone())
            }
            TokenKind::Dollar => {
                self.expect(TokenKind::Dollar)?;
                let identifier: String = self.expect(TokenKind::Identifier)?.literal.unwrap();
                Type::Generic(identifier, span.clone())
            }
            _ => {
                let identifier: String = self.expect(TokenKind::Identifier)?.literal.unwrap();
                if self.current()?.kind == TokenKind::Less {
                    self.expect(TokenKind::Less)?;
                    let mut types: Vec<Type> = vec![self.parse_type()?];
                    while self.current()?.kind == TokenKind::Comma {
                        self.expect(TokenKind::Comma)?;
                        types.push(self.parse_type()?);
                    }
                    self.expect(TokenKind::Greater)?;
                    return Ok(Type::GenericInstance(identifier, types, span));
                }
                Type::Unknown(identifier, span.clone())
            }
        };
        Ok(t)
    }

    fn peek(&mut self) -> Result<Token, BlazeError> {
        if self.current + 1 >= self.tokens.len() {
            return Err(BlazeError::ParseError(format!("unexpected end of file"), self.tokens[self.current].span.clone()));
        }
        Ok(self.tokens[self.current + 1].clone())
    }
    fn expect(&mut self, kind: TokenKind) -> Result<Token, BlazeError> {
        let token: Token = self.current()?;
        if token.kind != kind {
            return Err(BlazeError::ParseError(format!("unexpected token: {:?} ({:?}), expected: {:?}", token.kind, token.literal, kind), token.span.clone()));
        }
        self.advance()?;
        Ok(token)
    }
    fn current(&mut self) -> Result<Token, BlazeError> {
        if self.current >= self.tokens.len() {
            return Err(BlazeError::ParseError(format!("unexpected end of file"), self.tokens[self.current - 1].span.clone()));
        }
        let curr_token: Token = self.tokens[self.current].clone();
        if self.current < self.tokens.len() {
            return Ok(self.tokens[self.current].clone());
        }
        Err(BlazeError::ParseError(format!("unexpected end of file"), curr_token.span.clone()))
    }
    fn advance(&mut self) -> Result<(), BlazeError> {
        if self.current >= self.tokens.len() {
            return Err(BlazeError::ParseError(format!("unexpected end of file"), self.tokens[self.current - 1].span.clone()));
        }
        self.current += 1;
        Ok(())
    }
}