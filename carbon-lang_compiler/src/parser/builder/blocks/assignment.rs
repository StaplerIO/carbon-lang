use crate::shared::token::OperatorType;
use crate::shared::ast::action::AssignmentAction;
use crate::parser::utils::find_next_semicolon;
use crate::shared::ast::blocks::expression::Expression;
use crate::parser::builder::expression_builder::expression_infix_to_postfix;
use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};

pub fn assignment_block(tokens: Vec<DecoratedToken>) -> (Option<AssignmentAction>, isize) {
    let next_semicolon_pos = find_next_semicolon(tokens.clone());
    if next_semicolon_pos != -1 {
        if tokens[0].is_valid_identifier() && tokens[1].token_type == DecoratedTokenType::Operator {
            if tokens[1].operator.unwrap().operator_type == OperatorType::Assignment {
                // Convert expression
                let postfix_expr = expression_infix_to_postfix(tokens.clone()[2..(next_semicolon_pos as usize)].to_vec());

                return (Option::from(AssignmentAction {
                    identifier: tokens[0].data.clone().unwrap().clone().identifier.unwrap().clone(),
                    eval_expression: Expression { postfix_expr },
                }), next_semicolon_pos);
            }
        }
    }

    return (None, -1);
}
