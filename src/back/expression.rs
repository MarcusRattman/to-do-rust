#![allow(dead_code)]
use super::ExpressionError;

#[derive(Debug)]
pub enum LeftVar {
    Name,
    Description,
    Date,
    Category,
    Done,
}

#[derive(Debug)]
pub enum Op {
    Equals,
    NotEquals,
    Greater,
    Less,
    GrEquals,
    LeEquals,
    Like,
}

/// Matches the incoming string with the according [Task](crate::back::Task) field [LeftVar]
fn match_arg(arg: &str) -> Result<LeftVar, ExpressionError> {
    match arg {
        "name" => Ok(LeftVar::Name),
        "description" => Ok(LeftVar::Description),
        "date" => Ok(LeftVar::Date),
        "category" => Ok(LeftVar::Category),
        "status" => Ok(LeftVar::Done),
        _ => Err(ExpressionError::ArgParseError),
    }
}

/// Matches the incoming string with the according operator [Op]
///
/// Or returns [ExpressionError] if failed.
fn match_op(op: &str) -> Result<Op, ExpressionError> {
    match op {
        "<" => Ok(Op::Less),
        "<=" => Ok(Op::LeEquals),
        "=" => Ok(Op::Equals),
        "!=" => Ok(Op::NotEquals),
        ">" => Ok(Op::Greater),
        ">=" => Ok(Op::GrEquals),
        "like" => Ok(Op::Like),
        _ => Err(ExpressionError::OpParseError),
    }
}

/// Parses the expression [String] in format
///
/// `"arg=something"`
///
/// into a [Result] tuple: ([LeftVar], [Op], [String])
///
/// Or returns an [ExpressionError] when failed.
fn parse_expr(expression: String) -> Result<(LeftVar, Op, String), ExpressionError> {
    let op: String;

    if expression.contains("like") {
        op = "like".to_string();
    } else {
        // filters chars in string if they're in the list of supported operators
        // "arg=test" => "="
        // "arg=>test" => "=>"
        op = expression
            .chars()
            .filter(|c| match_op(&c.to_string()).is_ok())
            .collect();
    }

    // attempts to split arg into two parts by a separator
    let args: Vec<&str> = expression
        .split(&op)
        .map(|arg| arg.trim().trim_matches('"'))
        .filter(|arg| !arg.is_empty())
        .collect();

    let op = match_op(&op)?;

    if args.len() != 2 {
        return Err(ExpressionError::ExprParseError);
    }

    let (left, right) = (
        args.first().unwrap().to_string(),
        args.last().unwrap().to_string(),
    );

    let left = match_arg(&left)?;

    Ok((left, op, right))
}

/// Parses the string containing args into a [Vec] of tuples ([LeftVar], [Op], [String])
pub fn parse_args(args: &str) -> Result<Vec<(LeftVar, Op, String)>, ExpressionError> {
    let parsed: Vec<Result<(LeftVar, Op, String), ExpressionError>> = args
        .split("and")
        .map(|arg| arg.trim())
        .map(|arg| parse_expr(arg.to_string()))
        .collect();

    if parsed.iter().any(|arg| arg.is_err()) {
        return Err(ExpressionError::ExprParseError);
    }

    let unwrapped = parsed.into_iter().map(|arg| arg.unwrap()).collect();

    Ok(unwrapped)
}

/// Checks if first word in `input` is equal to `contains`
///
/// As an example the function is used in conjunction with
/// [TaskManager](crate::TaskManager)'s `exec_command` to determine if an incoming string is
/// a valid command
pub fn command_equals(command: &str, other: &str) -> Result<bool, ExpressionError> {
    let command = command.split_whitespace().collect::<Vec<&str>>();

    if let None = command.first() {
        return Err(ExpressionError::ArgParseError);
    }

    Ok(command.first().unwrap().trim().eq(other))
}
