#![allow(unused_must_use)]
use crate::{back::task_manager::TaskManager, back::task_model::Task};

#[test]
fn new_task_added() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    let test = mgr.add(task);
    assert!(test.is_ok());
}

#[test]
fn new_task_is_stored() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    mgr.add(task);
    let test = mgr.find("qwe");
    assert!(test.is_ok());
}

#[test]
fn check_done() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    mgr.add(task);
    mgr.check_done("qwe");
    let task = mgr.find("qwe").unwrap();
    assert!(task.done);
}

#[test]
fn check_done_wrong_task_name() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    mgr.add(task);
    let phantom_task_done = mgr.check_done("rty");
    assert!(phantom_task_done.is_err());
}

#[test]
fn delete() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    mgr.add(task);
    let deleted_result = mgr.delete("qwe");
    let find_result = mgr.find("qwe");
    assert!(deleted_result.is_ok() && find_result.is_err());
}

#[test]
fn delete_non_existent() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    mgr.add(task);
    let deleted_result = mgr.delete("rty");
    assert!(deleted_result.is_err());
}

#[test]
fn update() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    mgr.add(task);
    let (name, description, date, category) = ("zxc", "vbn", "2015-12-05 00:00", "iop");
    let updated = mgr.update("qwe", name, description, date, category);
    assert!(updated.is_ok());
}

#[test]
fn update_wrong_task_name() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    mgr.add(task);
    let (name, description, date, category) = ("zxc", "vbn", "2015-12-05 00:00", "iop");
    let updated = mgr.update("test", name, description, date, category);
    assert!(updated.is_err());
}

#[test]
fn update_values_updated() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    mgr.add(task);
    let (name, description, date, category) = ("zxc", "vbn", "2015-12-05 00:00", "iop");
    mgr.update("qwe", name, description, date, category);
    let updated = mgr.find("zxc");
    assert!(updated.is_ok());
}

#[test]
fn select() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    mgr.add(task);
    let query = "select * where name=qwe".to_string();
    let result = mgr.select(&query);

    assert!(result.is_ok());
}

#[test]
fn select_quotes() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    mgr.add(task);
    let query = "select * where name=\"qwe\"".to_string();
    let result = mgr.select(&query);

    assert!(result.is_ok());
}

#[test]
fn select_quotes_space() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe asd; rty; 2015-09-05 00:00; fgh").unwrap();
    mgr.add(task);
    let query = "select * where name=\"qwe asd\"".to_string();
    let result = mgr.select(&query);

    assert!(result.is_ok());
}

#[test]
fn select_quotes_like() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe asd; rty; 2015-09-05 00:00; fgh").unwrap();
    mgr.add(task);
    let query = "select * where name like \"qwe\"".to_string();
    let result = mgr.select(&query);

    assert!(result.is_ok());
}

#[test]
fn select_unwrap() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    mgr.add(task);
    let query = "select * where name=qwe".to_string();
    let result = mgr.select(&query).unwrap();

    assert_eq!(result[0].name, mgr.get_tasks()[0].name);
}

#[test]
fn select_error_gibberish() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    mgr.add(task);
    let query = "seasdasxzc aqwewnzxcv asdq".to_string();
    let result = mgr.select(&query);

    assert!(result.is_err());
}

#[test]
fn select_by_date() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    mgr.add(task);
    let query = "select * where date=2015-09-05 00:00".to_string();
    let result = mgr.select(&query);

    assert!(result.is_ok());
}

#[test]
fn select_multiple() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    let task2 = Task::new("zxc; asd; 2020-09-05 00:00; tyu").unwrap();
    mgr.add(task);
    mgr.add(task2);
    let query = "select *".to_string();
    let result = mgr.select(&query);

    assert!(result.is_ok());
}

#[test]
fn select_multiple_error() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    let task2 = Task::new("zxc; asd; 2020-09-05 00:00; tyu").unwrap();
    mgr.add(task);
    mgr.add(task2);
    let query = "select * where".to_string();
    let result = mgr.select(&query);

    assert!(result.is_err());
}

#[test]
fn select_from_multiple() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    let task2 = Task::new("zxc; asd; 2020-09-05 00:00; tyu").unwrap();
    mgr.add(task);
    mgr.add(task2);
    let query = "select * where name=zxc".to_string();
    let result = mgr.select(&query);

    assert!(result.is_ok());
}

#[test]
fn select_from_empty() {
    let mgr = TaskManager::new();
    let query = "select * where name=zxc".to_string();
    let result = mgr.select(&query);

    assert!(result.is_ok() && result.unwrap().len() == 0);
}

#[test]
fn select_from_multiple_len() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    let task2 = Task::new("zxc; asd; 2020-09-05 00:00; tyu").unwrap();
    mgr.add(task);
    mgr.add(task2);
    let query = "select * where name=zxc".to_string();
    let result = mgr.select(&query).unwrap();

    assert_eq!(result.len(), 1);
}

#[test]
fn select_from_multiple_not_found() {
    let mut mgr = TaskManager::new();
    let task = Task::new("qwe; rty; 2015-09-05 00:00; fgh").unwrap();
    let task2 = Task::new("zxc; asd; 2020-09-05 00:00; tyu").unwrap();
    mgr.add(task);
    mgr.add(task2);
    let query = "select * where name=gru".to_string();
    let result = mgr.select(&query).unwrap();

    assert_eq!(result.len(), 0);
}

#[test]
fn command_add() {
    let mut mgr = TaskManager::new();
    let command = "add qwe; rty; 2015-09-05 00:00; fgh";
    let result = mgr.exec_command(command);
    assert!(result.is_ok());
}

#[test]
fn command_add_many_args() {
    let mut mgr = TaskManager::new();
    let command = "add qwe; rty; 2015-09-05 00:00; fgh; asd";
    let result = mgr.exec_command(command);
    println!("\n\n{:?}\n\n", mgr.get_tasks());
    assert!(result.is_err());
}

#[test]
fn command_add_little_args() {
    let mut mgr = TaskManager::new();
    let command = "add qwe, rty";
    let result = mgr.exec_command(command);
    println!("\n\n{:?}\n\n", mgr.get_tasks());
    assert!(result.is_err());
}

#[test]
fn command_add_wrong_args() {
    let mut mgr = TaskManager::new();
    let command = "add ,,,qwe,, rty,";
    let result = mgr.exec_command(command);
    println!("\n\n{:?}\n\n", mgr.get_tasks());
    assert!(result.is_err());
}

#[test]
fn command_delete() {
    let mut mgr = TaskManager::new();
    mgr.exec_command("add qwe; rty; 2015-09-05 00:00; fgh");
    let result = mgr.exec_command("delete qwe");
    assert!(result.is_ok() && mgr.find("qwe").is_err());
}

#[test]
fn command_delete_wrong_args() {
    let mut mgr = TaskManager::new();
    mgr.exec_command("add qwe; rty; 2015-09-05 00:00; fgh");
    let result = mgr.exec_command("delete qweasdasdasdasd");
    assert!(result.is_err());
}

#[test]
fn command_wrong_command() {
    let mut mgr = TaskManager::new();
    let command = mgr.exec_command("zxcvzxcvzxcv");
    assert!(command.is_err());
}

#[test]
fn command_done() {
    let mut mgr = TaskManager::new();
    let add = "add qwe; rty; 2015-09-05 00:00; fgh";
    mgr.exec_command(add);

    let before = mgr.find("qwe").unwrap().done;
    mgr.exec_command("done qwe");
    let after = mgr.find("qwe").unwrap().done;

    assert_ne!(before, after);
}

#[test]
fn command_done_task_not_found() {
    let mut mgr = TaskManager::new();
    let add = "add qwe; rty; 2015-09-05 00:00; fgh";
    mgr.exec_command(add);
    let result = mgr.exec_command("done zxc");
    assert!(result.is_err());
}

#[test]
fn command_update() {
    let mut mgr = TaskManager::new();
    let add = "add qwe; rty; 2015-09-05 00:00; fgh";
    mgr.exec_command(add);
    let result = mgr.exec_command("update qwe; zxc; lmao; 2022-09-05 00:00; chores");
    assert!(result.is_ok());
}

#[test]
fn command_update_task_not_found() {
    let mut mgr = TaskManager::new();
    let add = "add qwe; rty; 2015-09-05 00:00; fgh";
    mgr.exec_command(add);
    let result = mgr.exec_command("update asd; zxc; lmao; 2022-09-05 00:00; chores");
    assert!(result.is_err());
}

#[test]
fn command_update_wrong_args() {
    let mut mgr = TaskManager::new();
    let add = "add qwe; rty; 2015-09-05 00:00; fgh";
    mgr.exec_command(add);
    let result = mgr.exec_command("update qwe; zxc");
    assert!(result.is_err());
}

#[test]
fn command_update_wrong_date() {
    let mut mgr = TaskManager::new();
    let add = "add qwe; rty; 2015-09-05 00:00; fgh";
    mgr.exec_command(add);
    let result = mgr.exec_command("update qwe; zxc; lmao; 2022-09-02; chores");
    assert!(result.is_err());
}

#[test]
fn command_select_by_name() {
    let mut mgr = TaskManager::new();
    let add = "add qwe; rty; 2015-09-05 00:00; fgh";
    mgr.exec_command(add);
    let result = mgr.exec_command("select * where name=qwe");
    assert!(result.is_ok());
}

#[test]
fn command_select_by_wrong_name() {
    let mut mgr = TaskManager::new();
    let add = "add qwe; rty; 2015-09-05 00:00; fgh";
    mgr.exec_command(add);
    let result = mgr.exec_command("select * where name=zxc");
    assert!(result.is_err());
}

#[test]
fn command_select_by_date() {
    let mut mgr = TaskManager::new();
    let add = "add qwe; rty; 2015-09-05 00:00; fgh";
    mgr.exec_command(add);
    let result = mgr.exec_command("select * where date=2015-09-05 00:00");
    assert!(result.is_ok());
}

#[test]
fn command_select_by_wrong_date() {
    let mut mgr = TaskManager::new();
    let add = "add qwe; rty; 2015-09-05 00:00; fgh";
    mgr.exec_command(add);
    let result = mgr.exec_command("select * where date=sdfrwe weq123");
    assert!(result.is_err());
}

#[test]
fn command_select_by_date_not_found() {
    let mut mgr = TaskManager::new();
    let add = "add qwe; rty; 2015-09-05 00:00; fgh";
    mgr.exec_command(add);
    let result = mgr.exec_command("select * where date=2022-09-05 00:00");
    assert!(result.is_err());
}

#[test]
fn select_partial_match() {
    let mut mgr = TaskManager::new();
    let task1 = Task::new("task1; desc1; 2015-09-05 00:00; cat1").unwrap();
    let task2 = Task::new("task2; desc2; 2015-09-06 00:00; cat2").unwrap();
    mgr.add(task1);
    mgr.add(task2);
    let select = "select * where name like \"task\"".to_string();
    let result = mgr.select(&select).unwrap();
    assert_eq!(result.len(), 2);
}
