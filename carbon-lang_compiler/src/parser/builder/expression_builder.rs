use crate::shared::token::{ContainerType, CalculationOperator, OperatorType};
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};

lazy_static! {
    /**
    * Operator priority:
    * calculation > relation > logical
    */
    static ref CALC_OPERATOR_PRIORITY: HashMap<CalculationOperator, u8> = [
        (CalculationOperator::Plus, 1),
        (CalculationOperator::Minus, 1),
        (CalculationOperator::Times, 2),
        (CalculationOperator::Divide, 2),
        (CalculationOperator::Mod, 2)
    ].iter().cloned().collect();

    static ref OPERATOR_PRIORITY: HashMap<OperatorType, u8> = [
        (OperatorType::Logical, 1),
        (OperatorType::Relation, 2),
        (OperatorType::Calculation, 3)
    ].iter().cloned().collect();
}

// We leave the postfix expression for code generator (solve the expression later)
pub fn expression_infix_to_postfix(tokens: Vec<DecoratedToken>) -> Vec<DecoratedToken> {
    let mut result: Vec<DecoratedToken> = Vec::new();
    let mut operator_stack: Vec<DecoratedToken> = Vec::new();

    for token in tokens {
        if token.token_type == DecoratedTokenType::Data {
            // Push all terms into result directly (infix to postfix)
            result.push(token.clone());
        } else if is_bracket(token.clone()) {
            // Which type of bracket? Anti bracket?
            if token.container.unwrap() == ContainerType::Bracket {
                // Push this bracket
                operator_stack.push(token.clone());
            } else {
                // Pop to result until the operator is a bracket (not anti-bracket)
                while operator_stack.last().unwrap().token_type != DecoratedTokenType::Container {
                    result.push(operator_stack.pop().unwrap());
                }

                // Pop this bracket (it won't be transferred to result)
                operator_stack.pop();
            }
        } else if is_operator(token.clone()) {
            while !operator_stack.is_empty() &&
                operator_stack.last().unwrap().token_type != DecoratedTokenType::Container {
                // Pop if operator priority is higher than current operator
                if priority_is_higher(operator_stack.last().unwrap().clone(), token.clone()) {
                    result.push(operator_stack.pop().unwrap());
                } else { break; }
            }

            operator_stack.push(token.clone());
        } else {
            panic!("Illegal token encountered!");
        }
    }

    // Push all operators remaining
    while !operator_stack.is_empty() {
        result.push(operator_stack.pop().unwrap());
    }

    return result;
}

fn is_operator(token: DecoratedToken) -> bool {
    if token.token_type == DecoratedTokenType::Operator {
        let operator = token.operator.unwrap();
        return operator.operator_type == OperatorType::Calculation ||
            operator.operator_type == OperatorType::Relation ||
            operator.operator_type == OperatorType::Logical;
    }

    return false;
}

fn is_bracket(token: DecoratedToken) -> bool {
    if token.token_type == DecoratedTokenType::Container {
        let container = token.container.unwrap();

        return container == ContainerType::Bracket ||
            container == ContainerType::AntiBracket;
    }

    return false;
}

// Return true if the priority of "a" is higher than or equal to "b"
fn priority_is_higher(a: DecoratedToken, b: DecoratedToken) -> bool {
    if is_operator(a.clone()) && is_operator(b.clone()) {
        return if a.operator.unwrap().operator_type != b.operator.unwrap().operator_type {
            OPERATOR_PRIORITY[&a.operator.unwrap().operator_type] >= OPERATOR_PRIORITY[&b.operator.unwrap().operator_type]
        } else if a.operator.unwrap().operator_type == OperatorType::Calculation {
            // Then they are equal on operator type (ElseIf ~ End If)
            // So we just need to compare with 1 token

            CALC_OPERATOR_PRIORITY[&a.operator.unwrap().calculation.unwrap()] >= CALC_OPERATOR_PRIORITY[&b.operator.unwrap().calculation.unwrap()]
        } else {
            true
        };
    }

    panic!("Token is not an operator!");
}
