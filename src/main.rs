pub mod token;

use std::collections::{HashMap, VecDeque};
use crate::token::*;
use crate::Value::{Float, Integer, Undefined};

enum Value {
	Undefined,
	Integer(i64),
	Float(f64)
}

struct Calculator {
	globals: HashMap<String, Value>
}
impl Calculator {
	fn new() -> Self {
		Self {
			globals: HashMap::new()
		}
	}
	fn calculate(src: &'static str) -> Result<Value, String> {
		let mut operator_stack = VecDeque::new();
		let mut output = VecDeque::new();
		let mut result = 0.0f64;

		match Token::tokenize(src) {
			Err(err) => { return Err(err); },
			Ok(tokens) => {
				for token in tokens {
					if token.is_operator() {
						if operator_stack.len() > 0 {
							match operator_stack.back().unwrap() {
								Token::UnaryOperator(_) => {
									output.push_back(operator_stack.pop_back().unwrap());
								},
								Token::BinaryOperator(opTypePrev) => {
									match token.clone() {
										Token::BinaryOperator(opType) => {
											if opTypePrev.order() > opType.order() {
												output.push_back(operator_stack.pop_back().unwrap());
											}
										}
										_ => { /* Do nothing */ }
									}
								}
								_ => { /* Do nothing */ }
							}
						}
						operator_stack.push_back(token);
					} else if token == Token::OpenParen {
						operator_stack.push_back(token);
					} else if token == Token::CloseParen {
						if operator_stack.len() == 0 {
							return Err(format!("Invalid token: '{}'", ')'));
						}

						let mut item = operator_stack.back().unwrap();
						while item != Token::OpenParen {
							output.push_back(operator_stack.pop_back().unwrap());
							if operator_stack.len() == 0 {
								return Err("Expected an opening parenthesis!".to_string());
							}
							item = operator_stack.back().unwrap();
						}
					} else {
						output.push_back(token);
					}
				}

				// TODO: Parse processed tokens

				return if result == result.round() {
					Ok(Integer(result.round() as i64))
				} else {
					Ok(Float(result))
				}
			}
		}
	}
}

fn main() {

}
