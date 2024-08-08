use crate::back::task_model::Task;

#[test]
fn task_created() {
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh");
    assert!(task.is_ok());
}

#[test]
fn new_task_wrong_args_less() {
    let test = Task::new("qwe rty");
    assert!(test.is_err());
}

#[test]
fn new_task_wrong_args_more() {
    let task = Task::new("qwe rty asd fgh zxc asd fgh zxc");
    assert!(task.is_err());
}

#[test]
fn task_updated() {
    let mut task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    let task_updated = task.update("name", "description", "2023-09-05 00:00", "cat");
    assert!(task_updated.is_ok());
}

#[test]
fn task_marked_done() {
    let mut task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    task.check_done();
    assert!(task.done);
}
