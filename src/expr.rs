use crate::scanner::*;

pub enum ExpLiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Nil,
}

use ExpLiteralValue::*;

fn un_wrap_as_f32(literal: Option<LiteralValue>) -> f32 {
    match literal {
	Some(LiteralValue::IntValue(x)) => x as f32,
	Some(LiteralValue::FloatValue(x)) => x as f32,
	_ => panic!("Could not unwrap as f32"),
    }
}

fn un_wrap_as_string(literal: Option<LiteralValue>) -> String {
    match literal {
	Some(LiteralValue::StringValue(x)) => x.clone(),
	Some(LiteralValue::IdentifierValue(x)) => x.clone(),
	_ => panic!("Could not unwrap as String"),
    }
}

impl ExpLiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            Number(x) => x.to_string(),
            StringValue(x) => x.clone(),
            True => "true".to_string(),
            False => "false".to_string(),
            Nil => "nil".to_string(),
        }
    }
    
    pub fn from_token(token: Token) -> Self {
	match token.token_type {
	    TokenType::Number => Self::Number(un_wrap_as_f32(token.literal)),
	    TokenType::StringLit => Self::StringValue(un_wrap_as_string(token.literal)),
	    TokenType::False => Self::False,
	    TokenType::True => Self::True,
	    TokenType::Nil => Self::Nil,
	    _ => panic!("Could not create LiteralValue from {:?}", token),
	}
    }
}


pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: ExpLiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                operator.lexeme,
                left.to_string(),
                right.to_string(),
            ),
            Expr::Grouping { expression } => format!("(group {})", expression.to_string()),
            Expr::Literal { value } => format!("{}", value.to_string()),
            Expr::Unary { operator, right } => {
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string();
                format!("({} {})", operator_str, right_str)
            }
        }
    }

    pub fn from_token(&self) {
        println!("{}", self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::Expr::*;
    use super::*;

    #[test]
    fn pretty_print_ast() {
        let minus_token = Token {
            token_type: TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            line_number: 0,
        };
        let number = Literal {
            value: Number(123.0),
        };
        let group = Grouping {
            expression: Box::from(Literal {
                value: Number(45.67),
            }),
        };
        let multiplication = Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line_number: 0,
        };

        let ast = Binary {
            left: Box::from(Unary {
                operator: minus_token,
                right: Box::from(number),
            }),
            operator: multiplication,
            right: Box::from(group),
        };

	let result = ast.to_string();
	assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}
