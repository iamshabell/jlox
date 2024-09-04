use crate::expr::Expr::*;
use crate::expr::Expr;
use crate::expr::ExpLiteralValue;
use crate::scanner::*;
use crate::scanner::TokenType::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

macro_rules! try_match {
    ($instance: expr, $($token: ident),*) => {
        {
	    let mut result = false;
	    {
		$(
		    result |= $instance.try_match($token);
		)+
	    }
	    
	    result
	}
    };
}


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
	Self {
	    tokens,
	    current: 0,
	}
    }

    fn expression(&mut self) -> Expr {
	self.equality()
    }

    fn equality(&mut self) -> Expr {
	let mut expr = self.comparison();
	while try_match!(self, BangEqual, EqualEqual) {
	   let operator = self.previous();
	   let rhs = self.comparison();
	    
	    expr = Binary { left: Box::from(expr), operator, right: Box::from(rhs) }
	}

	expr
    }


    fn comparison(&mut self) -> Expr {
	let mut expr = self.term();
	while try_match!(self, Greater, GreaterEqual, Less, LessEqual) {
	    let operator = self.previous();
	    let rhs = self.term();
	    
	    expr = Binary { left: Box::from(expr), operator, right: Box::from(rhs) }
	}

	expr
    }

    fn term(&mut self) -> Expr {
	let mut expr = self.factor();
	while try_match!(self, Minus, Plus) {
	    let operator = self.previous();
	    let rhs = self.term();
	    
	    expr = Binary { left: Box::from(expr), operator, right: Box::from(rhs) }
	}

	expr
    }

     fn factor(&mut self) -> Expr {
	let mut  expr = self.unary();
	while try_match!(self, Slash, Star) {
	    let operator = self.previous();
	    let rhs = self.term();
	    
	    expr = Binary { left: Box::from(expr), operator, right: Box::from(rhs) }
	}

	expr
    }
    
    fn unary(&mut self) -> Expr {
	if try_match!(self, Bang, Minus) {
	    let operator = self.previous();
	    let rhs = self.term();
	    
	    Unary { operator, right: Box::from(rhs) }
	} else {
	    self.primary()
	}
    }

    fn primary(&mut self) -> Expr {
	if self.try_match(LeftParen) {
	    let expr = self.expression();
	    self.consume(RightParen, "Expect ')' after expression.");
	    Grouping { expression: Box::from(expr) }
	} else {
	    let token = self.peek();
	    Literal { value: ExpLiteralValue::from_token(token) }
	}
    }

    fn consume(&self, token_type: TokenType, msg: &str) {
	todo!()
    }

    fn try_match(&self, token_type: TokenType) -> bool {
	if self.is_at_end() {
	    return false
	}

	self.peek().token_type == token_type
    }

    fn advance(&mut self ) -> Token {
	if self.is_at_end() {
	    self.current += 1;
	}
	
	self.previous()
    }

    fn peek(&self) -> Token {
	self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
	self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
	self.peek().token_type == Eof
    }

}
