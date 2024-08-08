#![allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TaskMgrError {
    TaskCreationErrorProp(TaskError),
    TaskNotFound,
    TaskUpdateError,
    TaskUpdateErrorPropTask(TaskError),
    TaskDeleteError,
    WrongQuery,
    WrongQueryPropExpr(ExpressionError),
    WrongCommand,
}

#[derive(Debug, Clone)]
pub enum TaskError {
    TaskDateParseError,
    TaskUpdateError,
    TaskCreationArgsError,
}

#[derive(Debug, Clone)]
pub enum ExpressionError {
    LikeParseError,
    ExprParseError,
    ArgParseError,
    OpParseError,
}
