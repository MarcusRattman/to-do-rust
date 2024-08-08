use crate::back::expression::parse_args;

#[test]
pub fn parse_expr() {
    let args = "name=test1";
    let parsed = parse_args(args);
    assert!(parsed.is_ok());
}

#[test]
pub fn parse_error_empty() {
    let args = "";
    let parsed = parse_args(args);
    assert!(parsed.is_err());
}

#[test]
pub fn parse_error_empty_whitespace() {
    let args = " ";
    let parsed = parse_args(args);
    assert!(parsed.is_err());
}

#[test]
pub fn parse_error_empty_whitespaces() {
    let args = "  ";
    let parsed = parse_args(args);
    assert!(parsed.is_err());
}

#[test]
pub fn parse_like() {
    let args = "name like test1";
    let parsed = parse_args(args);
    assert!(parsed.is_ok());
}

#[test]
pub fn parse_both() {
    let args = "name=test1 and description like something";
    let parsed = parse_args(args);
    assert!(parsed.is_ok());
}

#[test]
pub fn parse_error_gibberish() {
    let args = "asdasdasd";
    let parsed = parse_args(args);
    assert!(parsed.is_err());
}

#[test]
pub fn parse_error_wrong_arg_right() {
    let args = "arg=";
    let parsed = parse_args(args);
    assert!(parsed.is_err());
}

#[test]
pub fn parse_error_wrong_arg_left() {
    let args = "=arg";
    let parsed = parse_args(args);
    assert!(parsed.is_err());
}

#[test]
pub fn parse_error_pseudo_arg() {
    let args = "arg!arg";
    let parsed = parse_args(args);
    assert!(parsed.is_err());
}

#[test]
pub fn parse_error_gibberish_with_and() {
    let args = "asdasdasd and";
    let parsed = parse_args(args);
    assert!(parsed.is_err());
}

#[test]
pub fn parse_error_gibberish_with_two_args() {
    let args = "asdas and dasd";
    let parsed = parse_args(args);
    assert!(parsed.is_err());
}

#[test]
pub fn parse_error_gibberish_with_arg() {
    let args = "asdasdasd and test=test";
    let parsed = parse_args(args);
    assert!(parsed.is_err());
}

#[test]
pub fn parse_error_gibberish_with_like() {
    let args = "asdasdasd like";
    let parsed = parse_args(args);
    assert!(parsed.is_err());
}
