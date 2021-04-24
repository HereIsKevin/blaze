use crate::token::Token;

#[derive(Clone, Debug)]
pub struct Literal {
    pub name: Token,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub parameters: Vec<Variant>,
    pub output: Option<Variant>,
}

#[derive(Clone, Debug)]
pub enum Variant {
    Literal(Box<Literal>),
    Function(Box<Function>),
}

impl Variant {
    pub fn new_literal(name: Token) -> Self {
        Self::Literal(Box::new(Literal { name }))
    }

    pub fn new_function(
        parameters: Vec<Variant>,
        output: Option<Variant>,
    ) -> Self {
        Self::Function(Box::new(Function { parameters, output }))
    }

    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
        match self {
            Self::Literal(variant) => visitor.visit_literal_variant(variant),
            Self::Function(variant) => visitor.visit_function_variant(variant),
        }
    }
}

pub trait Visitor {
    type Result;

    fn visit_literal_variant(&mut self, variant: &Literal) -> Self::Result;
    fn visit_function_variant(&mut self, variant: &Function) -> Self::Result;
}
