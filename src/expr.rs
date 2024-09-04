use crate::scanner::*;

pub enum LiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Nil,
}

use LiteralValue::*;

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            Number(x) => x.to_string(),
            StringValue(x) => x.clone(),
            True => "true".to_string(),
            False => "false".to_string(),
            Nil => "nil".to_string(),
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
        value: LiteralValue,
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

    pub fn print(&self) {
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
