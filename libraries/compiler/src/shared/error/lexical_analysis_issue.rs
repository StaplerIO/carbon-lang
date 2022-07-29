use std::fmt::{Display, Formatter};
use crate::shared::error::general_issue::FileMatch;

#[derive(Debug, Clone)]
pub struct LexicalAnalysisIssue {
    pub location: FileMatch
}
