use crate::error::SyntaxError;
use crate::expr::Expr;
use crate::kind::Kind;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::value::Value;
use crate::variant::Variant;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> (Vec<Stmt>, Vec<SyntaxError>) {
        let mut statements = Vec::new();
        let mut errors = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(error) => {
                    self.synchronize();
                    errors.push(error);
                }
            }
        }

        (statements, errors)
    }

    fn declaration(&mut self) -> Result<Stmt, SyntaxError> {
        if self.compare(&[Kind::Fn]) {
            self.function_declaration()
        } else if self.compare(&[Kind::Type]) {
            self.type_declaration()
        } else {
            Err(self.error(self.peek(), "Expect function or type declaration."))
        }
    }

    fn function_declaration(&mut self) -> Result<Stmt, SyntaxError> {
        let name = self
            .consume(Kind::Identifier, "Expect function name.")?
            .clone();

        self.consume(Kind::LeftParen, "Expect '(' after function name.")?;

        let mut parameters = Vec::new();

        if !self.check(Kind::RightParen) {
            let name = self
                .consume(Kind::Identifier, "Expect parameter name.")?
                .clone();

            self.consume(Kind::Colon, "Expect ':' after parameter name.")?;

            let variant = self.variant()?;

            parameters.push((name, variant));

            while self.compare(&[Kind::Comma]) {
                let name = self
                    .consume(Kind::Identifier, "Expect parameter name.")?
                    .clone();

                self.consume(Kind::Colon, "Expect ':' after parameter name.")?;

                let variant = self.variant()?;

                parameters.push((name, variant));
            }

            self.compare(&[Kind::Comma]);
        }

        self.consume(Kind::RightParen, "Expect ')' after parameters.")?;

        let output = if self.compare(&[Kind::Colon]) {
            Some(self.variant()?)
        } else {
            None
        };

        self.consume(Kind::LeftBrace, "Expect '{' before function body.")?;

        let body = self.block_statement()?;

        Ok(Stmt::new_function(name, parameters, output, body))
    }

    fn type_declaration(&mut self) -> Result<Stmt, SyntaxError> {
        let name = self.consume(Kind::Identifier, "Expect type name.")?.clone();
        self.consume(Kind::Equal, "Expect '=' after type name.")?;
        let variant = self.variant()?;
        self.consume(Kind::Semicolon, "Expect ';' after type.")?;

        Ok(Stmt::new_type(name, variant))
    }

    fn statement(&mut self) -> Result<Stmt, SyntaxError> {
        if self.compare(&[Kind::If]) {
            self.if_statement()
        } else if self.compare(&[Kind::Return]) {
            self.return_statement()
        } else if self.compare(&[Kind::Loop]) {
            self.loop_statement()
        } else if self.compare(&[Kind::Break]) {
            self.break_statement()
        } else if self.compare(&[Kind::Continue]) {
            self.continue_statement()
        } else if self.compare(&[Kind::Let]) {
            self.let_statement()
        } else if self.compare(&[Kind::LeftBrace]) {
            self.block_statement()
        } else {
            self.assignment_statement()
        }
    }

    fn if_statement(&mut self) -> Result<Stmt, SyntaxError> {
        let condition = self.expression()?;
        self.consume(Kind::LeftBrace, "Expect block after 'if'.")?;

        let then_branch = self.block_statement()?;

        let else_branch = if self.compare(&[Kind::Else]) {
            self.consume(Kind::LeftBrace, "Expect block after 'else'.")?;
            Some(self.block_statement()?)
        } else {
            None
        };

        Ok(Stmt::new_if(condition, then_branch, else_branch))
    }

    fn return_statement(&mut self) -> Result<Stmt, SyntaxError> {
        let value = if self.check(Kind::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(Kind::Semicolon, "Expect ';' after return value.")?;

        Ok(Stmt::new_return(value))
    }

    fn loop_statement(&mut self) -> Result<Stmt, SyntaxError> {
        self.consume(Kind::LeftBrace, "Expect '{' after 'loop'.")?;
        let body = self.block_statement()?;

        Ok(Stmt::new_loop(body))
    }

    fn break_statement(&mut self) -> Result<Stmt, SyntaxError> {
        self.consume(Kind::Semicolon, "Expect ';' after 'break'.")?;

        Ok(Stmt::new_break())
    }

    fn continue_statement(&mut self) -> Result<Stmt, SyntaxError> {
        self.consume(Kind::Semicolon, "Expect ';' after 'continue'.")?;

        Ok(Stmt::new_continue())
    }

    fn let_statement(&mut self) -> Result<Stmt, SyntaxError> {
        let name = self
            .consume(Kind::Identifier, "Expect variable name.")?
            .clone();

        self.consume(Kind::Colon, "Expect variable type.")?;
        let variant = self.variant()?;

        let initializer = if self.compare(&[Kind::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            Kind::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::new_let(name, variant, initializer))
    }

    fn block_statement(&mut self) -> Result<Stmt, SyntaxError> {
        let mut statements = Vec::new();

        while !self.is_at_end() && !self.check(Kind::RightBrace) {
            statements.push(self.statement()?);
        }

        self.consume(Kind::RightBrace, "Expect '}' after block.")?;

        Ok(Stmt::new_block(statements))
    }

    fn assignment_statement(&mut self) -> Result<Stmt, SyntaxError> {
        let expr = self.expression()?;

        if self.compare(&[Kind::Equal]) {
            let equals = self.previous().clone();
            let value = self.expression()?;

            if let Expr::Variable(variable) = expr {
                self.consume(Kind::Semicolon, "Expect ';' after assignment.")?;

                Ok(Stmt::new_assignment(variable.name, value))
            } else {
                Err(self.error(&equals, "Invalid assignment target."))
            }
        } else {
            self.consume(Kind::Semicolon, "Expect ';' after expression.")?;

            Ok(Stmt::new_expression(expr))
        }
    }

    fn expression(&mut self) -> Result<Expr, SyntaxError> {
        self.or_expression()
    }

    fn or_expression(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.and_expression()?;

        while self.compare(&[Kind::BarBar]) {
            let operator = self.previous().clone();
            let right = self.and_expression()?;

            expr = Expr::new_logical(expr, operator, right);
        }

        Ok(expr)
    }

    fn and_expression(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.equality_expression()?;

        while self.compare(&[Kind::AmpAmp]) {
            let operator = self.previous().clone();
            let right = self.equality_expression()?;

            expr = Expr::new_logical(expr, operator, right);
        }

        Ok(expr)
    }

    fn equality_expression(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.comparison_expression()?;

        while self.compare(&[Kind::BangEqual, Kind::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison_expression()?;

            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn comparison_expression(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.term_expression()?;

        while self.compare(&[
            Kind::Greater,
            Kind::GreaterEqual,
            Kind::Less,
            Kind::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term_expression()?;

            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn term_expression(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.factor_expression()?;

        while self.compare(&[Kind::Minus, Kind::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor_expression()?;

            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn factor_expression(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.unary_expression()?;

        while self.compare(&[Kind::Slash, Kind::Star]) {
            let operator = self.previous().clone();
            let right = self.unary_expression()?;

            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn unary_expression(&mut self) -> Result<Expr, SyntaxError> {
        if self.compare(&[Kind::Bang, Kind::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary_expression()?;

            Ok(Expr::new_unary(operator, right))
        } else {
            self.call_expression()
        }
    }

    fn call_expression(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.primary_expression()?;

        while self.compare(&[Kind::LeftParen]) {
            let mut arguments = Vec::new();

            if !self.check(Kind::RightParen) {
                arguments.push(self.expression()?);

                while self.compare(&[Kind::Comma]) {
                    arguments.push(self.expression()?);
                }

                self.compare(&[Kind::Comma]);
            }

            self.consume(Kind::RightParen, "Expect ')' after arguments.")?;

            expr = Expr::new_call(expr, arguments);
        }

        Ok(expr)
    }

    fn primary_expression(&mut self) -> Result<Expr, SyntaxError> {
        if self.compare(&[Kind::False, Kind::True, Kind::Number, Kind::String])
        {
            let token = self.previous();
            let literal = match token.kind {
                Kind::False => Value::False,
                Kind::True => Value::True,
                Kind::Number => Value::Number(token.lexeme.clone()),
                Kind::String => {
                    let mut characters = token.lexeme.chars();
                    characters.next();
                    characters.next_back();

                    Value::String(characters.collect())
                }
                _ => return Err(self.error(token, "Parser bug, wrong literal")),
            };

            Ok(Expr::new_literal(literal))
        } else if self.compare(&[Kind::Identifier]) {
            Ok(Expr::new_variable(self.previous().clone()))
        } else if self.compare(&[Kind::LeftParen]) {
            let expr = self.expression()?;
            self.consume(Kind::RightParen, "Expect ')' after expression.")?;

            Ok(Expr::new_grouping(expr))
        } else {
            Err(self.error(self.peek(), "Expect expression."))
        }
    }

    fn variant(&mut self) -> Result<Variant, SyntaxError> {
        if self.compare(&[Kind::Identifier]) {
            Ok(self.literal_variant()?)
        } else if self.compare(&[Kind::Fn]) {
            Ok(self.function_variant()?)
        } else {
            Err(self.error(self.peek(), "Expect literal or function type."))
        }
    }

    fn literal_variant(&mut self) -> Result<Variant, SyntaxError> {
        Ok(Variant::new_literal(self.previous().clone()))
    }

    fn function_variant(&mut self) -> Result<Variant, SyntaxError> {
        self.consume(Kind::LeftParen, "Expect '(' after function type.")?;

        let mut parameters = Vec::new();

        if !self.check(Kind::RightParen) {
            parameters.push(self.variant()?);

            while self.compare(&[Kind::Comma]) {
                parameters.push(self.variant()?)
            }

            self.compare(&[Kind::Comma]);
        }

        self.consume(Kind::RightParen, "Expect ')' after function type.")?;

        let output = if self.compare(&[Kind::Colon]) {
            Some(self.variant()?)
        } else {
            None
        };

        Ok(Variant::new_function(parameters, output))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if matches!(self.peek().kind, Kind::Fn | Kind::Type) {
                return;
            }

            self.advance();
        }
    }

    fn consume(
        &mut self,
        kind: Kind,
        message: &str,
    ) -> Result<&Token, SyntaxError> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(self.error(self.peek(), message))
        }
    }

    fn error(&self, token: &Token, message: &str) -> SyntaxError {
        let location = match token.kind {
            Kind::EOF => " at end".to_string(),
            _ => format!(" at '{}'", token.lexeme),
        };

        SyntaxError {
            line: token.line,
            location,
            message: message.to_string(),
        }
    }

    fn compare(&mut self, kinds: &[Kind]) -> bool {
        for kind in kinds.iter() {
            if self.check(*kind) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, kind: Kind) -> bool {
        !self.is_at_end() && self.peek().kind == kind
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, Kind::EOF)
    }
}
