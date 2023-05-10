use crate::{error::BlazeError, ast::{Statement, Type, EnumVariant, Expression}, scope::Scope};

#[derive(Debug, Clone)]
pub struct Typechecker {
    pub statements: Vec<Statement>,
    pub errors: Vec<BlazeError>,
    pub current_scope: Scope,
}

impl Typechecker {
    pub fn new(statements: Vec<Statement>) -> Typechecker {
        Typechecker {
            statements,
            errors: Vec::new(),
            current_scope: Scope::new(None),
        }
    }

    pub fn typecheck(&mut self) -> Result<(), Vec<BlazeError>> {
        for statement in self.statements.clone() {
            let result: Result<(), BlazeError> = self.typecheck_statement(statement.clone());
            if let Err(error) = result {
                self.errors.push(error);
            }
        }

        if self.errors.len() > 0 {
            Err(self.errors.clone())
        } else {
            Ok(())
        }
    }

    fn typecheck_statement(&mut self, statement: Statement) -> Result<(), BlazeError> {
        match statement {
            Statement::Namespace(identifier, statements, _) => {
                self.current_scope.namespaces.insert(identifier, statements.clone());
                self.open_new_scope();
                for statement in statements {
                    let result: Result<(), BlazeError> = self.typecheck_statement(statement);
                    if let Err(error) = result {
                        self.errors.push(error);
                    }
                }
                self.close_current_scope();
                Ok(())
            }
            Statement::Struct(identifier, _, fields, _) => {
                self.current_scope.structs.insert(identifier.clone(), fields.clone());
                self.open_new_scope();
                for field in fields {
                    let ty: Type = field.ty.clone();
                    let result: Result<(), BlazeError> = self.typecheck_type(ty);
                    if let Err(error) = result {
                        self.errors.push(error);
                    }
                }
                self.close_current_scope();
                Ok(())
            }
            Statement::Enum(identifier, variants, _) => {
                self.current_scope.enums.insert(identifier.clone(), variants.clone());
                Ok(())
            }
            Statement::TypedEnum(identifier, base_ty, variants, _) => {
                self.current_scope.enums.insert(identifier.clone(), variants.clone());
                let result: Result<(), BlazeError> = self.typecheck_type(base_ty);
                if let Err(error) = result {
                    self.errors.push(error);
                }
                Ok(())
            }
            Statement::Union(identifier, types, _) => {
                self.current_scope.unions.insert(identifier.clone(), types.clone());
                self.open_new_scope();
                for ty in types {
                    let result: Result<(), BlazeError> = self.typecheck_type(ty);
                    if let Err(error) = result {
                        self.errors.push(error);
                    }
                }
                self.close_current_scope();
                Ok(())
            }
            Statement::Function(identifier, parameters, return_type, body, _) => {
                self.current_scope.functions.insert(identifier.clone(), (
                    parameters.clone().into_iter().map(|(identifier, ty, _, _)| (identifier, ty.clone())).collect(),
                    return_type.clone()
                ));
                self.open_new_scope();
                for parameter in parameters {
                    let ty: Type = parameter.1.clone();
                    let result: Result<(), BlazeError> = self.typecheck_type(ty);
                    if let Err(error) = result {
                        self.errors.push(error);
                    }
                    self.current_scope.parameters.insert(parameter.0.clone(), (parameter.1.clone(), parameter.2.clone(), parameter.3.clone()));
                }
                for statement in body {
                    let result: Result<(), BlazeError> = self.typecheck_statement(statement);
                    if let Err(error) = result {
                        self.errors.push(error);
                    }
                }
                self.close_current_scope();
                Ok(())
            }
            Statement::StructFunction(_, identifier, parameters, return_type, body, _) => {
                self.current_scope.functions.insert(identifier.clone(), (
                    parameters.clone().into_iter().map(|(identifier, ty, _, _)| (identifier, ty.clone())).collect(),
                    return_type.clone()
                ));
                self.open_new_scope();
                for parameter in parameters {
                    let ty: Type = parameter.1.clone();
                    let result: Result<(), BlazeError> = self.typecheck_type(ty);
                    if let Err(error) = result {
                        self.errors.push(error);
                    }
                    self.current_scope.parameters.insert(parameter.0.clone(), (parameter.1.clone(), parameter.2.clone(), parameter.3.clone()));
                }
                for statement in body {
                    let result: Result<(), BlazeError> = self.typecheck_statement(statement);
                    if let Err(error) = result {
                        self.errors.push(error);
                    }
                }
                self.close_current_scope();
                Ok(())
            }
            Statement::ConstantDeclaration(identifier, ty, expression, _) => {
                self.current_scope.consts.insert(identifier.clone(), (ty.clone(), expression.clone()));
                let result: Result<(), BlazeError> = self.typecheck_type(ty);
                if let Err(error) = result {
                    self.errors.push(error);
                }
                let result: Result<(), BlazeError> = self.typecheck_expression(expression);
                if let Err(error) = result {
                    self.errors.push(error);
                }
                Ok(())
            }
            Statement::VariableDeclaration(identifier, ty, expression, _) => {
                self.current_scope.variables.insert(identifier.clone(), (ty.clone(), expression.clone()));
                let result: Result<(), BlazeError> = self.typecheck_type(ty.clone());
                if let Err(error) = result {
                    self.errors.push(error);
                }
                let result: Result<(), BlazeError> = self.typecheck_expression(expression.clone());
                if let Err(error) = result {
                    self.errors.push(error);
                }
                Ok(())
            }
            Statement::MutableDeclaration(identifier, ty, expression, _) => {
                self.current_scope.mutables.insert(identifier.clone(), (ty.clone(), expression.clone()));
                let result: Result<(), BlazeError> = self.typecheck_type(ty);
                if let Err(error) = result {
                    self.errors.push(error);
                }
                let result: Result<(), BlazeError> = self.typecheck_expression(expression);
                if let Err(error) = result {
                    self.errors.push(error);
                }
                Ok(())
            }
            Statement::Return(expressions, _) => {
                if expressions.len() > 0 {
                    for expression in expressions {
                        let result: Result<(), BlazeError> = self.typecheck_expression(expression);
                        if let Err(error) = result {
                            self.errors.push(error);
                        }
                    }
                }
                Ok(())
            }
            Statement::Expression(expression, _) => {
                let result: Result<(), BlazeError> = self.typecheck_expression(expression);
                if let Err(error) = result {
                    self.errors.push(error);
                }
                Ok(())
            }
            Statement::While(condition, body, _) => {
                let result: Result<(), BlazeError> = self.typecheck_expression(condition);
                if let Err(error) = result {
                    self.errors.push(error);
                }
                self.open_new_scope();
                for statement in body {
                    let result: Result<(), BlazeError> = self.typecheck_statement(statement);
                    if let Err(error) = result {
                        self.errors.push(error);
                    }
                }
                self.close_current_scope();
                Ok(())
            }
            _ => Err(BlazeError::TypeError(format!("unexpected statement"), statement.span())),
        }
    }

    fn typecheck_expression(&mut self, expression: Expression) -> Result<(), BlazeError> {
        match expression {
            Expression::Identifier(_, _) => Ok(()),
            Expression::Integer(_, _) => Ok(()),
            Expression::Char(_, _) => Ok(()),
            Expression::String(_, _) => Ok(()),
            Expression::SelfLiteral(_) => Ok(()),
            Expression::StructLiteral(identifier, values, span) => {
                if let Some(fields) = self.current_scope.structs.get(&identifier) {
                    for (field, value) in fields.clone().into_iter().zip(values.clone()) {
                        let ty: Type = field.ty.clone();
                        let result: Result<(), BlazeError> = self.typecheck_type(ty);
                        if let Err(error) = result {
                            self.errors.push(error);
                        }
                        let result: Result<(), BlazeError> = self.typecheck_expression(value.1);
                        if let Err(error) = result {
                            self.errors.push(error);
                        }
                    }
                    Ok(())
                } else {
                    Err(BlazeError::TypeError(format!("unknown struct '{}'", identifier), span))
                }
            }
            Expression::StaticMemberAccess(expression, member, _) => {
                let result: Result<(), BlazeError> = self.typecheck_expression(*expression.clone());
                if let Err(error) = result {
                    self.errors.push(error);
                }
                let result: Result<(), BlazeError> = self.typecheck_expression(*member.clone());
                if let Err(error) = result {
                    self.errors.push(error);
                }
                Ok(())
            }
            Expression::MemberAccess(expression, member, _) => {
                let result: Result<(), BlazeError> = self.typecheck_expression(*expression.clone());
                if let Err(error) = result {
                    self.errors.push(error);
                }
                let result: Result<(), BlazeError> = self.typecheck_expression(*member.clone());
                if let Err(error) = result {
                    self.errors.push(error);
                }
                Ok(())
            }
            Expression::BinaryOperation(lhs, _, rhs, _) => {
                let result: Result<(), BlazeError> = self.typecheck_expression(*lhs.clone());
                if let Err(error) = result {
                    self.errors.push(error);
                }
                let result: Result<(), BlazeError> = self.typecheck_expression(*rhs.clone());
                if let Err(error) = result {
                    self.errors.push(error);
                }
                Ok(())
            }
            Expression::ArrayAccess(identifier, index, span) => {
                if let Some(ty) = self.current_scope.locate(identifier.clone(), span.clone()) {
                    let result: Result<(), BlazeError> = self.typecheck_type(ty.0);
                    if let Err(error) = result {
                        self.errors.push(error);
                    }
                } else {
                    return Err(BlazeError::TypeError(format!("unknown identifier '{}'", identifier), span));
                }
                let result: Result<(), BlazeError> = self.typecheck_expression(*index.clone());
                if let Err(error) = result {
                    self.errors.push(error);
                }
                Ok(())
            }
            Expression::AddressOf(expr, _) => {
                let result: Result<(), BlazeError> = self.typecheck_expression(*expr.clone());
                if let Err(error) = result {
                    self.errors.push(error);
                }
                Ok(())
            }
            Expression::Dereference(expr, _) => {
                let result: Result<(), BlazeError> = self.typecheck_expression(*expr.clone());
                if let Err(error) = result {
                    self.errors.push(error);
                }
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    fn typecheck_type(&mut self, ty: Type) -> Result<(), BlazeError> {
        match ty {
            Type::I8(_) => Ok(()),
            Type::I16(_) => Ok(()),
            Type::I32(_) => Ok(()),
            Type::I64(_) => Ok(()),
            Type::U8(_) => Ok(()),
            Type::U16(_) => Ok(()),
            Type::U32(_) => Ok(()),
            Type::U64(_) => Ok(()),
            Type::F32(_) => Ok(()),
            Type::F64(_) => Ok(()),
            Type::Char(_) => Ok(()),
            Type::Bool(_) => Ok(()),
            Type::Void(_) => Ok(()),
            Type::Array(ty, _) => self.typecheck_type(*ty),
            Type::Pointer(ty, _) => self.typecheck_type(*ty),
            Type::Optional(ty, _) => self.typecheck_type(*ty),
            Type::VarArgs(ty, _) => {
                if let Some(ty) = *ty {
                    self.typecheck_type(ty)
                } else {
                    Ok(())
                }
            }
            Type::Unknown(identifier, span) => {
                if let Some(ref ty) = self.current_scope.get_type(identifier.clone(), span.clone()) {
                    self.typecheck_type(ty.clone())
                } else {
                    Err(BlazeError::TypeError(format!("unknown type '{}'", identifier), span))
                }
            }
            Type::Struct(identifier, span) => {
                if let Some(fields) = self.current_scope.structs.get(&identifier) {
                    for field in fields.clone() {
                        let ty: Type = field.ty.clone();
                        let result: Result<(), BlazeError> = self.typecheck_type(ty);
                        if let Err(error) = result {
                            self.errors.push(error);
                        }
                    }
                    Ok(())
                } else {
                    Err(BlazeError::TypeError(format!("unknown struct '{}'", identifier), span))
                }
            }
            Type::Enum(identifier, span) => {
                if let Some(variants) = self.current_scope.enums.get(&identifier) {
                    for variant in variants.clone() {
                        match variant {
                            EnumVariant::Unit(_, _) => (),
                            EnumVariant::Expression(_, expression, _) => {
                                let result: Result<(), BlazeError> = self.typecheck_expression(expression);
                                if let Err(error) = result {
                                    self.errors.push(error);
                                }
                            }
                        }
                    }
                    Ok(())
                } else {
                    Err(BlazeError::TypeError(format!("unknown enum '{}'", identifier), span))
                }
            }
            Type::Union(identifier, span) => {
                if let Some(types) = self.current_scope.unions.get(&identifier) {
                    for ty in types.clone() {
                        let result: Result<(), BlazeError> = self.typecheck_type(ty);
                        if let Err(error) = result {
                            self.errors.push(error);
                        }
                    }
                    Ok(())
                } else {
                    Err(BlazeError::TypeError(format!("unknown union '{}'", identifier), span))
                }
            }
            _ => Err(BlazeError::TypeError(format!("unexpected type '{:?}'", ty), ty.span())),
        }
    }

    fn open_new_scope(&mut self) {
        let previous_scope = self.current_scope.clone();
        self.current_scope = Scope::new(Some(previous_scope.clone()));
        previous_scope.namespaces.clone().into_iter().for_each(|(key, value)| { self.current_scope.namespaces.insert(key, value); });
        previous_scope.structs.clone().into_iter().for_each(|(key, value)| { self.current_scope.structs.insert(key, value); });
        previous_scope.enums.clone().into_iter().for_each(|(key, value)| { self.current_scope.enums.insert(key, value); });
        previous_scope.unions.clone().into_iter().for_each(|(key, value)| { self.current_scope.unions.insert(key, value); });
        previous_scope.functions.clone().into_iter().for_each(|(key, value)| { self.current_scope.functions.insert(key, value); });
        previous_scope.consts.clone().into_iter().for_each(|(key, value)| { self.current_scope.consts.insert(key, value); });
        previous_scope.variables.clone().into_iter().for_each(|(key, value)| { self.current_scope.variables.insert(key, value); });
        previous_scope.mutables.clone().into_iter().for_each(|(key, value)| { self.current_scope.mutables.insert(key, value); });
    }

    fn close_current_scope(&mut self) {
        self.current_scope = self.current_scope.parent.clone().unwrap();
    }
}