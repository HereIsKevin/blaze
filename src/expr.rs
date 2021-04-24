use crate::token::Token;
use crate::value::Value;

#[derive(Clone, Debug)]
pub struct Logical {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

#[derive(Clone, Debug)]
pub struct Binary {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

#[derive(Clone, Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Expr,
}

#[derive(Clone, Debug)]
pub struct Call {
    pub callee: Expr,
    pub arguments: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub struct Grouping {
    pub expression: Expr,
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub name: Token,
}

#[derive(Clone, Debug)]
pub struct Literal {
    pub value: Value,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Logical(Box<Logical>),
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Call(Box<Call>),
    Grouping(Box<Grouping>),
    Variable(Box<Variable>),
    Literal(Box<Literal>),
}

impl Expr {
    pub fn new_logical(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Logical(Box::new(Logical {
            left,
            operator,
            right,
        }))
    }

    pub fn new_binary(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Binary(Box::new(Binary {
            left,
            operator,
            right,
        }))
    }

    pub fn new_unary(operator: Token, right: Expr) -> Self {
        Self::Unary(Box::new(Unary { operator, right }))
    }

    pub fn new_call(callee: Expr, arguments: Vec<Expr>) -> Self {
        Self::Call(Box::new(Call { callee, arguments }))
    }

    pub fn new_grouping(expression: Expr) -> Self {
        Self::Grouping(Box::new(Grouping { expression }))
    }

    pub fn new_variable(name: Token) -> Self {
        Self::Variable(Box::new(Variable { name }))
    }

    pub fn new_literal(value: Value) -> Self {
        Self::Literal(Box::new(Literal { value }))
    }

    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
        match self {
            Self::Logical(expr) => visitor.visit_logical_expr(expr),
            Self::Binary(expr) => visitor.visit_binary_expr(expr),
            Self::Unary(expr) => visitor.visit_unary_expr(expr),
            Self::Call(expr) => visitor.visit_call_expr(expr),
            Self::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Self::Variable(expr) => visitor.visit_variable_expr(expr),
            Self::Literal(expr) => visitor.visit_literal_expr(expr),
        }
    }
}

pub trait Visitor {
    type Result;

    fn visit_logical_expr(&mut self, expr: &Logical) -> Self::Result;
    fn visit_binary_expr(&mut self, expr: &Binary) -> Self::Result;
    fn visit_unary_expr(&mut self, expr: &Unary) -> Self::Result;
    fn visit_call_expr(&mut self, expr: &Call) -> Self::Result;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Self::Result;
    fn visit_variable_expr(&mut self, expr: &Variable) -> Self::Result;
    fn visit_literal_expr(&mut self, expr: &Literal) -> Self::Result;
}
