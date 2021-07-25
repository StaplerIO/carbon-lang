use crate::shared::ast::blocks::expression::Expression;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ActionType {
    DeclarationStatement,
    AssignmentStatement,
    CallStatement,
    ReturnStatement,
    IfStatement,
    WhileStatement,
    LoopStatement,
    SwitchStatement,
    BreakStatement,
    ContinueStatement,
    EmptyAction
}

#[derive(Clone)]
pub struct Action {
    pub action_type: ActionType,

    pub declaration_action: Option<DeclarationAction>,
    pub assignment_action: Option<AssignmentAction>,

    pub call_action: Option<CallAction>,
    pub return_action: Option<ReturnAction>,
    pub if_action: Option<IfAction>,
    pub while_action: Option<ConditionBlock>,
    pub loop_action: Option<ActionBlock>,
    pub switch_action: Option<SwitchAction>,

    // "break" and "continue" actions don't have special blocks
}

#[derive(Clone)]
pub struct ActionBlock {
    pub actions: Vec<Action>
}

// Used in while, if, elif
#[derive(Clone)]
pub struct ConditionBlock {
    pub condition: Expression,
    pub body: ActionBlock
}

#[derive(Clone)]
pub struct DeclarationAction {
    // A variable or a constant
    pub is_variable: bool,

    pub identifier: String,
    pub data_type: String
}

#[derive(Clone)]
pub struct AssignmentAction {
    pub identifier: String,
    pub eval_expression: Expression
}

#[derive(Clone)]
pub struct CallAction {
    pub function_name: String,
    // Arguments are Expressions
    pub arguments: Vec<Expression>
}

#[derive(Clone)]
pub struct ReturnAction {
    pub value: Expression
}

#[derive(Clone)]
pub struct IfAction {
    pub if_block: ConditionBlock,

    pub elif_collection: Vec<ConditionBlock>,

    pub else_action: Option<ActionBlock>
}

// TODO: Builder required
#[derive(Clone)]
pub struct SwitchAction {
    pub condition: Expression,
    pub cases: Vec<SwitchCase>
}

// Builder with SwitchAction
#[derive(Clone)]
pub struct SwitchCase {
    pub is_default: bool,
    pub value: String,
    pub actions: ActionBlock
}
