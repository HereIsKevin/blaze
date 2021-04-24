use std::mem;

use crate::error::GenerateError;
use crate::expr;
use crate::kind::Kind;
use crate::stmt;
use crate::value::Value;
use crate::variant;

static RUNTIME: &str = r#"
    #![allow(dead_code, unused_mut, unused_parens)]

    use std::fmt::Display;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn clock() -> f64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }

    fn print(value: impl Display) {
        println!("{}", value);
    }
"#;

pub struct Generator {
    errors: Vec<GenerateError>,
}

impl Generator {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn generate(
        &mut self,
        statements: &[stmt::Stmt],
    ) -> (String, Vec<GenerateError>) {
        let generated: Vec<String> = statements
            .iter()
            .map(|statement| statement.accept(self))
            .collect();

        let output = format!("{}{}", RUNTIME, generated.join(" "));
        let errors = mem::take(&mut self.errors);

        (output, errors)
    }

    fn error(&mut self, line: usize, message: &str) -> String {
        self.errors.push(GenerateError {
            line,
            message: message.to_string(),
        });

        "()".to_string()
    }
}

impl expr::Visitor for Generator {
    type Result = String;

    fn visit_logical_expr(&mut self, expr: &expr::Logical) -> Self::Result {
        let operator = match expr.operator.kind {
            Kind::AmpAmp => "&&",
            Kind::BarBar => "||",
            _ => return self.error(expr.operator.line, "Unexpected operator."),
        };

        format!(
            "({} {} {})",
            expr.left.accept(self),
            operator,
            expr.right.accept(self)
        )
    }

    fn visit_binary_expr(&mut self, expr: &expr::Binary) -> Self::Result {
        let operator = match expr.operator.kind {
            Kind::BangEqual => "!=",
            Kind::EqualEqual => "==",
            Kind::LessEqual => "<=",
            Kind::Less => "<",
            Kind::GreaterEqual => ">=",
            Kind::Greater => ">",
            Kind::Plus => "+",
            Kind::Minus => "-",
            Kind::Star => "*",
            Kind::Slash => "/",
            _ => return self.error(expr.operator.line, "Unexpected operator."),
        };

        format!(
            "({} {} {})",
            expr.left.accept(self),
            operator,
            expr.right.accept(self)
        )
    }

    fn visit_unary_expr(&mut self, expr: &expr::Unary) -> Self::Result {
        let operator = match expr.operator.kind {
            Kind::Minus => "-",
            Kind::Bang => "!",
            _ => return self.error(expr.operator.line, "Unexpected operator."),
        };

        format!("({}{})", operator, expr.right.accept(self))
    }

    fn visit_call_expr(&mut self, expr: &expr::Call) -> Self::Result {
        let arguments: Vec<String> = expr
            .arguments
            .iter()
            .map(|argument| argument.accept(self))
            .collect();

        format!("({})({})", expr.callee.accept(self), arguments.join(", "))
    }

    fn visit_grouping_expr(&mut self, expr: &expr::Grouping) -> Self::Result {
        format!("({})", expr.expression.accept(self))
    }

    fn visit_variable_expr(&mut self, expr: &expr::Variable) -> Self::Result {
        expr.name.lexeme.clone()
    }

    fn visit_literal_expr(&mut self, expr: &expr::Literal) -> Self::Result {
        match &expr.value {
            Value::False => "false".to_string(),
            Value::True => "true".to_string(),
            Value::Number(number) => number.to_string(),
            Value::String(string) => format!("\"{}\"", string),
        }
    }
}

impl stmt::Visitor for Generator {
    type Result = String;

    fn visit_if_stmt(&mut self, stmt: &stmt::If) -> Self::Result {
        let else_branch = if let Some(branch) = &stmt.else_branch {
            format!(" else {{ {} }}", branch.accept(self))
        } else {
            "".to_string()
        };

        format!(
            "if {} {{ {} }}{}",
            stmt.condition.accept(self),
            stmt.then_branch.accept(self),
            else_branch
        )
    }

    fn visit_function_stmt(&mut self, stmt: &stmt::Function) -> Self::Result {
        let parameters: Vec<String> = stmt
            .parameters
            .iter()
            .map(|parameter| {
                format!("{}: {}", parameter.0.lexeme, parameter.1.accept(self))
            })
            .collect();

        let output = if let Some(variant) = &stmt.output {
            variant.accept(self)
        } else {
            "()".to_string()
        };

        format!(
            "fn {}({}) -> {} {}",
            stmt.name.lexeme,
            parameters.join(", "),
            output,
            stmt.body.accept(self)
        )
    }

    fn visit_return_stmt(&mut self, stmt: &stmt::Return) -> Self::Result {
        if let Some(expression) = &stmt.value {
            format!("return {};", expression.accept(self))
        } else {
            "return;".to_string()
        }
    }

    fn visit_loop_stmt(&mut self, stmt: &stmt::Loop) -> Self::Result {
        format!("loop {}", stmt.body.accept(self))
    }

    fn visit_break_stmt(&mut self, _stmt: &stmt::Break) -> Self::Result {
        "break;".to_string()
    }

    fn visit_continue_stmt(&mut self, _stmt: &stmt::Continue) -> Self::Result {
        "continue;".to_string()
    }

    fn visit_let_stmt(&mut self, stmt: &stmt::Let) -> Self::Result {
        let initializer = if let Some(expression) = &stmt.initializer {
            format!(" = {}", expression.accept(self))
        } else {
            "".to_string()
        };

        format!(
            "let mut {}: {}{};",
            stmt.name.lexeme,
            stmt.variant.accept(self),
            initializer
        )
    }

    fn visit_type_stmt(&mut self, stmt: &stmt::Type) -> Self::Result {
        format!("type {} = {};", stmt.name.lexeme, stmt.variant.accept(self))
    }

    fn visit_block_stmt(&mut self, stmt: &stmt::Block) -> Self::Result {
        let statements: Vec<String> = stmt
            .statements
            .iter()
            .map(|statement| statement.accept(self))
            .collect();

        format!("{{ {} }}", statements.join(" "))
    }

    fn visit_assignment_stmt(
        &mut self,
        stmt: &stmt::Assignment,
    ) -> Self::Result {
        format!("{} = {};", stmt.name.lexeme, stmt.value.accept(self))
    }

    fn visit_expression_stmt(
        &mut self,
        stmt: &stmt::Expression,
    ) -> Self::Result {
        format!("{};", stmt.expression.accept(self))
    }
}

impl variant::Visitor for Generator {
    type Result = String;

    fn visit_literal_variant(
        &mut self,
        variant: &variant::Literal,
    ) -> Self::Result {
        variant.name.lexeme.clone()
    }

    fn visit_function_variant(
        &mut self,
        variant: &variant::Function,
    ) -> Self::Result {
        let parameters: Vec<String> = variant
            .parameters
            .iter()
            .map(|parameter| parameter.accept(self))
            .collect();

        let output = if let Some(variant) = &variant.output {
            variant.accept(self)
        } else {
            "()".to_string()
        };

        format!("fn({}) -> {}", parameters.join("\n"), output)
    }
}
