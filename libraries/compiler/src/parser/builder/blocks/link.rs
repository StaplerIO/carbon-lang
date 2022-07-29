use crate::parser::utils::find_next_semicolon;
use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::error::general_issue::{GeneralIssue, IssueBase, IssueLevel, IssuePosition};

pub fn link_statement_builder(
    tokens: &Vec<DecoratedToken>,
) -> Result<(String, usize), GeneralIssue<String>> {
    if tokens.len() >= 3 {
        let next_semicolon_pos = find_next_semicolon(tokens.clone());

        if next_semicolon_pos.unwrap_or(0) == 2 {
            if tokens[0].content.get_decorated_keyword().is_some()
                && tokens[1].content.is_valid_identifier()
            {
                return Ok((tokens[1].content.get_data().unwrap().get_identifier().unwrap().clone(), 2));
            }
        }
    }

    return Err(GeneralIssue {
        issues: vec![IssueBase {
            level: IssueLevel::Info,
            position: IssuePosition::Parsing,
            code: "".to_string(),
            detail: "".to_string(),
        }]
    });
}
