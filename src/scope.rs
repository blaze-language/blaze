use std::collections::HashMap;
use crate::{ast::{Expression, Type, Statement, StructField, EnumVariant}, span::Span};

#[derive(Debug, Clone)]
pub struct Scope {
    pub parent: Box<Option<Scope>>,
    pub namespaces: HashMap<String, Vec<Statement>>,
    pub structs: HashMap<String, Vec<StructField>>,
    pub enums: HashMap<String, Vec<EnumVariant>>,
    pub unions: HashMap<String, Vec<Type>>,
    pub functions: HashMap<String, (Vec<(String, Type)>, Vec<Type>)>,
    pub consts: HashMap<String, (Type, Expression)>,
    pub variables: HashMap<String, (Type, Expression)>,
    pub mutables: HashMap<String, (Type, Expression)>,
    pub parameters: HashMap<String, (Type, Span)>,
}

impl Scope {
    pub fn new(parent: Option<Scope>) -> Scope {
        Scope {
            parent: Box::new(parent),
            namespaces: HashMap::new(),
            structs: HashMap::new(),
            enums: HashMap::new(),
            unions: HashMap::new(),
            functions: HashMap::new(),
            consts: HashMap::new(),
            variables: HashMap::new(),
            mutables: HashMap::new(),
            parameters: HashMap::new(),
        }
    }

    pub fn locate(&self, identifier: String, span: Span) -> Option<(Type, Expression)> {
        for (const_id, (ty, expr)) in self.consts.clone() {
            if const_id == identifier {
                return Some((ty, expr));
            }
        }
        for (var_id, (ty, expr)) in self.variables.clone() {
            if var_id == identifier {
                return Some((ty, expr));
            }
        }
        for (mut_id, (ty, expr)) in self.mutables.clone() {
            if mut_id == identifier {
                return Some((ty, expr));
            }
        }
        for (param_id, (ty, _)) in self.parameters.clone() {
            if param_id == identifier {
                return Some((ty, Expression::Identifier(identifier, span)));
            }
        }
        if let Some(parent) = &*self.parent {
            return parent.locate(identifier, span);
        }
        None
    }

    pub fn get_type(&self, identifier: String, span: Span) -> Option<Type> {
        for (struct_id, _) in self.structs.clone() {
            if struct_id == identifier {
                return Some(Type::Struct(struct_id, span));
            }
        }
        for (enum_id, _) in self.enums.clone() {
            if enum_id == identifier {
                return Some(Type::Enum(enum_id, span));
            }
        }
        for (union_id, _) in self.unions.clone() {
            if union_id == identifier {
                return Some(Type::Union(union_id, span));
            }
        }
        None
    }
}