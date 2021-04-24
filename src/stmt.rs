use crate::expr::Expr;
use crate::token::Token;
use crate::variant::Variant;

#[derive(Clone, Debug)]
pub struct If {
    pub condition: Expr,
    pub then_branch: Stmt,
    pub else_branch: Option<Stmt>,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: Token,
    pub parameters: Vec<(Token, Variant)>,
    pub output: Option<Variant>,
    pub body: Stmt,
}

#[derive(Clone, Debug)]
pub struct Return {
    pub value: Option<Expr>,
}

#[derive(Clone, Debug)]
pub struct Loop {
    pub body: Stmt,
}

#[derive(Clone, Debug)]
pub struct Break {}

#[derive(Clone, Debug)]
pub struct Continue {}

#[derive(Clone, Debug)]
pub struct Let {
    pub name: Token,
    pub variant: Variant,
    pub initializer: Option<Expr>,
}

#[derive(Clone, Debug)]
pub struct Type {
    pub name: Token,
    pub variant: Variant,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub name: Token,
    pub value: Expr,
}

#[derive(Clone, Debug)]
pub struct Expression {
    pub expression: Expr,
}

#[derive(Clone, Debug)]
pub enum Stmt {
    If(Box<If>),
    Function(Box<Function>),
    Return(Box<Return>),
    Loop(Box<Loop>),
    Break(Box<Break>),
    Continue(Box<Continue>),
    Let(Box<Let>),
    Type(Box<Type>),
    Block(Box<Block>),
    Assignment(Box<Assignment>),
    Expression(Box<Expression>),
}

impl Stmt {
    pub fn new_if(
        condition: Expr,
        then_branch: Stmt,
        else_branch: Option<Stmt>,
    ) -> Self {
        Self::If(Box::new(If {
            condition,
            then_branch,
            else_branch,
        }))
    }

    pub fn new_function(
        name: Token,
        parameters: Vec<(Token, Variant)>,
        output: Option<Variant>,
        body: Stmt,
    ) -> Self {
        Self::Function(Box::new(Function {
            name,
            parameters,
            output,
            body,
        }))
    }

    pub fn new_return(value: Option<Expr>) -> Self {
        Self::Return(Box::new(Return { value }))
    }

    pub fn new_loop(body: Stmt) -> Self {
        Self::Loop(Box::new(Loop { body }))
    }

    pub fn new_break() -> Self {
        Self::Break(Box::new(Break {}))
    }

    pub fn new_continue() -> Self {
        Self::Continue(Box::new(Continue {}))
    }

    pub fn new_let(
        name: Token,
        variant: Variant,
        initializer: Option<Expr>,
    ) -> Self {
        Self::Let(Box::new(Let {
            name,
            variant,
            initializer,
        }))
    }

    pub fn new_type(name: Token, variant: Variant) -> Self {
        Self::Type(Box::new(Type { name, variant }))
    }

    pub fn new_block(statements: Vec<Stmt>) -> Self {
        Self::Block(Box::new(Block { statements }))
    }

    pub fn new_assignment(name: Token, value: Expr) -> Self {
        Self::Assignment(Box::new(Assignment { name, value }))
    }

    pub fn new_expression(expression: Expr) -> Self {
        Self::Expression(Box::new(Expression { expression }))
    }

    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
        match self {
            Self::If(stmt) => visitor.visit_if_stmt(stmt),
            Self::Function(stmt) => visitor.visit_function_stmt(stmt),
            Self::Return(stmt) => visitor.visit_return_stmt(stmt),
            Self::Loop(stmt) => visitor.visit_loop_stmt(stmt),
            Self::Break(stmt) => visitor.visit_break_stmt(stmt),
            Self::Continue(stmt) => visitor.visit_continue_stmt(stmt),
            Self::Let(stmt) => visitor.visit_let_stmt(stmt),
            Self::Type(stmt) => visitor.visit_type_stmt(stmt),
            Self::Block(stmt) => visitor.visit_block_stmt(stmt),
            Self::Assignment(stmt) => visitor.visit_assignment_stmt(stmt),
            Self::Expression(stmt) => visitor.visit_expression_stmt(stmt),
        }
    }
}

pub trait Visitor {
    type Result;

    fn visit_if_stmt(&mut self, stmt: &If) -> Self::Result;
    fn visit_function_stmt(&mut self, stmt: &Function) -> Self::Result;
    fn visit_return_stmt(&mut self, stmt: &Return) -> Self::Result;
    fn visit_loop_stmt(&mut self, stmt: &Loop) -> Self::Result;
    fn visit_break_stmt(&mut self, stmt: &Break) -> Self::Result;
    fn visit_continue_stmt(&mut self, stmt: &Continue) -> Self::Result;
    fn visit_let_stmt(&mut self, stmt: &Let) -> Self::Result;
    fn visit_type_stmt(&mut self, stmt: &Type) -> Self::Result;
    fn visit_block_stmt(&mut self, stmt: &Block) -> Self::Result;
    fn visit_assignment_stmt(&mut self, stmt: &Assignment) -> Self::Result;
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> Self::Result;
}
