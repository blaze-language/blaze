use crate::{span::Span, token::TokenKind};

#[derive(Debug, Clone)]
pub enum Statement {
    ConstantDeclaration(String, Type, Expression, Span),
    VariableDeclaration(String, Type, Expression, Span),
    MutableDeclaration(String, Type, Expression, Span),
    Namespace(String, Vec<Statement>, Span),
    Import(String, String, Span),
    Struct(String, Vec<String>, Vec<StructField>, Span),
    Enum(String, Vec<EnumVariant>, Span),
    TypedEnum(String, Type, Vec<EnumVariant>, Span),
    Union(String, Vec<Type>, Span),
    Function(String, Vec<(String, Type, bool, Span)>, Vec<Type>, Vec<Statement>, Span),
    StructFunction(Type, String, Vec<(String, Type, bool, Span)>, Vec<Type>, Vec<Statement>, Span),
    Return(Vec<Expression>, Span),
    While(Expression, Vec<Statement>, Span),
    If(Expression, Vec<Statement>, Vec<Statement>, Span),
    Expression(Expression, Span),
}

#[derive(Debug, Clone)]
pub enum Expression {
    SelfLiteral(Span),
    Identifier(String, Span),
    Integer(i64, Span),
    Char(char, Span),
    String(String, Span),
    StaticMemberAccess(Box<Expression>, Box<Expression>, Span),
    MemberAccess(Box<Expression>, Box<Expression>, Span),
    BinaryOperation(Box<Expression>, TokenKind, Box<Expression>, Span),
    ArrayAccess(String, Box<Expression>, Span),
    StructLiteral(String, Vec<(Option<String>, Expression, Span)>, Span),
    AddressOf(Box<Expression>, Span),
    Dereference(Box<Expression>, Span),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I8(Span), I16(Span), I32(Span), I64(Span),
    U8(Span), U16(Span), U32(Span), U64(Span),
    F32(Span), F64(Span),
    Char(Span),
    Bool(Span),
    Void(Span),

    Array(Box<Type>, Span),
    Pointer(Box<Type>, Span),
    Optional(Box<Type>, Span),

    VarArgs(Box<Option<Type>>, Span),

    Generic(String, Span),

    Unknown(String, Span),
    Struct(String, Span),
    Enum(String, Span),
    Union(String, Span),

    AwaitingInference,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum EnumVariant {
    Unit(String, Span),
    Expression(String, Expression, Span),
}

impl Statement {
    pub fn span(&self) -> Span {
        match self {
            Statement::ConstantDeclaration(_, _, _, s) => s.clone(),
            Statement::VariableDeclaration(_, _, _, s) => s.clone(),
            Statement::MutableDeclaration(_, _, _, s) => s.clone(),
            Statement::Import(_, _, s) => s.clone(),
            Statement::Namespace(_, _, s) => s.clone(),
            Statement::Struct(_, _, _, s) => s.clone(),
            Statement::Enum(_, _, s) => s.clone(),
            Statement::TypedEnum(_, _, _, s) => s.clone(),
            Statement::Union(_, _, s) => s.clone(),
            Statement::Function(_, _, _, _, s) => s.clone(),
            Statement::StructFunction(_, _, _, _, _, s) => s.clone(),
            Statement::Return(_, s) => s.clone(),
            Statement::While(_, _, s) => s.clone(),
            Statement::If(_, _, _, s) => s.clone(),
            Statement::Expression(_, s) => s.clone(),
        }
    }
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::SelfLiteral(s) => s.clone(),
            Expression::Identifier(_, s) => s.clone(),
            Expression::Integer(_, s) => s.clone(),
            Expression::Char(_, s) => s.clone(),
            Expression::String(_, s) => s.clone(),
            Expression::StaticMemberAccess(_, _, s) => s.clone(),
            Expression::MemberAccess(_, _, s) => s.clone(),
            Expression::BinaryOperation(_, _, _, s) => s.clone(),
            Expression::ArrayAccess(_, _, s) => s.clone(),
            Expression::StructLiteral(_, _, s) => s.clone(),
            Expression::AddressOf(_, s) => s.clone(),
            Expression::Dereference(_, s) => s.clone(),
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Expression::Identifier(ident, s) => Type::Unknown(ident.clone(), s.clone()),
            Expression::Integer(_, s) => Type::I64(s.clone()),
            Expression::Char(_, s) => Type::Char(s.clone()),
            Expression::String(_, s) => Type::Pointer(Box::new(Type::Char(s.clone())), s.clone()),
            Expression::StaticMemberAccess(_, member, _) => member.get_type(),
            Expression::MemberAccess(_, member, _) => member.get_type(),
            Expression::StructLiteral(name, _, s) => Type::Unknown(name.clone(), s.clone()),
            Expression::AddressOf(expr, s) => Type::Pointer(Box::new(expr.get_type()), s.clone()),
            Expression::Dereference(expr, _) => match expr.get_type() {
                Type::Pointer(ty, _) => *ty,
                _ => unreachable!("Expression::get_type()"),
            },
            _ => unreachable!("Expression::get_type()"),
        }
    }
}

impl Type {
    pub fn span(&self) -> Span {
        match self {
            Type::I8(s) => s.clone(),
            Type::I16(s) => s.clone(),
            Type::I32(s) => s.clone(),
            Type::I64(s) => s.clone(),
            Type::U8(s) => s.clone(),
            Type::U16(s) => s.clone(),
            Type::U32(s) => s.clone(),
            Type::U64(s) => s.clone(),
            Type::F32(s) => s.clone(),
            Type::F64(s) => s.clone(),
            Type::Char(s) => s.clone(),
            Type::Bool(s) => s.clone(),
            Type::Void(s) => s.clone(),
            Type::Array(_, s) => s.clone(),
            Type::Pointer(_, s) => s.clone(),
            Type::Optional(_, s) => s.clone(),
            Type::VarArgs(_, s) => s.clone(),
            Type::Generic(_, s) => s.clone(),
            Type::Unknown(_, s) => s.clone(),
            Type::Struct(_, s) => s.clone(),
            Type::Enum(_, s) => s.clone(),
            Type::Union(_, s) => s.clone(),
            Type::AwaitingInference => unreachable!(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Type::I8(_) => "i8".to_string(),
            Type::I16(_) => "i16".to_string(),
            Type::I32(_) => "i32".to_string(),
            Type::I64(_) => "i64".to_string(),
            Type::U8(_) => "u8".to_string(),
            Type::U16(_) => "u16".to_string(),
            Type::U32(_) => "u32".to_string(),
            Type::U64(_) => "u64".to_string(),
            Type::F32(_) => "f32".to_string(),
            Type::F64(_) => "f64".to_string(),
            Type::Char(_) => "char".to_string(),
            Type::Bool(_) => "bool".to_string(),
            Type::Void(_) => "void".to_string(),
            Type::Array(ty, _) => format!("{}[]", ty.name()),
            Type::Pointer(ty, _) => format!("{}*", ty.name()),
            Type::Optional(ty, _) => format!("{}?", ty.name()),
            Type::VarArgs(ty, _) => {
                if let Some(ty) = *ty.clone() {
                    format!("...{}", ty.name())
                } else {
                    "...".to_string()
                }
            }
            Type::Generic(name, _) => format!("${}", name.clone()),
            Type::Unknown(name, _) => format!("unknown {}", name.clone()),
            Type::Struct(name, _) => format!("struct {}", name.clone()),
            Type::Enum(name, _) => format!("enum {}", name.clone()),
            Type::Union(name, _) => format!("union {}", name.clone()),
            Type::AwaitingInference => unreachable!(),
        }
    }

    pub fn equals(&self, other: Type) -> bool {
        match self {
            Type::I8(_) => matches!(other, Type::I8(_)),
            Type::I16(_) => matches!(other, Type::I16(_)),
            Type::I32(_) => matches!(other, Type::I32(_)),
            Type::I64(_) => matches!(other, Type::I64(_)),
            Type::U8(_) => matches!(other, Type::U8(_)),
            Type::U16(_) => matches!(other, Type::U16(_)),
            Type::U32(_) => matches!(other, Type::U32(_)),
            Type::U64(_) => matches!(other, Type::U64(_)),
            Type::F32(_) => matches!(other, Type::F32(_)),
            Type::F64(_) => matches!(other, Type::F64(_)),
            Type::Char(_) => matches!(other, Type::Char(_)),
            Type::Bool(_) => matches!(other, Type::Bool(_)),
            Type::Void(_) => matches!(other, Type::Void(_)),
            Type::Array(ty, _) => {
                if let Type::Array(other_ty, _) = other {
                    ty.equals(*other_ty)
                } else {
                    false
                }
            }
            Type::Pointer(ty, _) => {
                if let Type::Pointer(other_ty, _) = other {
                    ty.equals(*other_ty)
                } else {
                    false
                }
            }
            Type::Optional(ty, _) => {
                if let Type::Optional(other_ty, _) = other {
                    ty.equals(*other_ty)
                } else {
                    false
                }
            }
            Type::Unknown(name, _) => {
                if let Type::Unknown(other_name, _) = other {
                    name.clone() == other_name
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}