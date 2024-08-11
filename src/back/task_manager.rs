#![allow(dead_code)]
use super::expression::{command_equals, parse_args, LeftVar, Op};
use super::task_model::parse_task_date;
use super::{Task, TaskMgrError};
use std::fmt::Display;

#[derive(Debug)]
pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
    /// Matches the incoming text command in form of:
    ///
    /// `"add name, description, 2222-12-12 00:00, category"`
    ///
    /// and decides what to do with it
    pub fn exec_command(&mut self, command: &str) -> Result<String, TaskMgrError> {
        match command {
            _ if command_equals(command, "add").unwrap() => {
                let command = command.strip_prefix("add").unwrap().trim();
                let task = Task::new(command);
                if let Ok(t) = task {
                    self.add(t).unwrap();
                    return Ok("Task added".to_string());
                }
                Err(TaskMgrError::TaskCreationErrorProp(task.unwrap_err()))
            }
            _ if command_equals(command, "delete").unwrap() => {
                let command = command.strip_prefix("delete").unwrap().trim();
                let deleted = self.delete(command);
                if let Ok(_) = deleted {
                    return Ok(format!("Task deleted"));
                }
                Err(deleted.unwrap_err())
            }
            _ if command_equals(command, "select").unwrap() => {
                let selected = self.select(&command.to_string());
                if let Ok(result) = selected {
                    if result.is_empty() {
                        return Err(TaskMgrError::TaskNotFound);
                    }
                    return Ok(format!("Selected: {:?}", result));
                }
                Err(selected.unwrap_err())
            }
            _ if command_equals(command, "done").unwrap() => {
                let command = command.strip_prefix("done").unwrap().trim();
                let marked_done = self.check_done(command);
                if let Ok(result) = marked_done {
                    return Ok(format!("Task marked as done: {:?}", result));
                }
                Err(marked_done.unwrap_err())
            }
            // update old_name, new_name, descrip, date, cat
            _ if command_equals(command, "update").unwrap() => {
                // old_name new_name, descrip, date, cat
                let command = command.strip_prefix("update").unwrap().trim();
                // [old_name, new_name, descrip, date, cat]
                let composed: Vec<&str> = command.split(";").map(|field| field.trim()).collect();
                if composed.len() != 5 {
                    return Err(TaskMgrError::WrongQuery);
                }

                let updated = self.update(
                    &composed[0],
                    &composed[1],
                    &composed[2],
                    &composed[3],
                    &composed[4],
                );

                if let Ok(_) = updated {
                    return Ok(format!("Task updated"));
                }

                Err(updated.unwrap_err())
            }
            _ => Err(TaskMgrError::WrongCommand),
        }
    }

    pub fn find(&self, task_name: &str) -> Result<&Task, TaskMgrError> {
        self.tasks
            .iter()
            .find(|task| task.name == task_name)
            .ok_or(TaskMgrError::TaskNotFound)
    }

    pub fn add(&mut self, task: Task) -> Result<(), TaskMgrError> {
        self.tasks.push(task);
        Ok(())
    }

    pub fn update(
        &mut self,
        task_name: &str,
        name: &str,
        desc: &str,
        date: &str,
        cat: &str,
    ) -> Result<(), TaskMgrError> {
        let task = self.tasks.iter_mut().find(|task| task.name == task_name);
        if let Some(task) = task {
            if let Err(e) = task.update(name, desc, date, cat) {
                return Err(TaskMgrError::TaskUpdateErrorPropTask(e));
            }

            return Ok(());
        }

        Err(TaskMgrError::TaskUpdateError)
    }

    pub fn check_done(&mut self, task_name: &str) -> Result<&Task, TaskMgrError> {
        let task = self.tasks.iter_mut().find(|task| task.name.eq(task_name));
        if let Some(e) = task {
            e.check_done();
            return Ok(e);
        } else {
            return Err(TaskMgrError::TaskNotFound);
        }
    }

    pub fn delete(&mut self, task_name: &str) -> Result<(), TaskMgrError> {
        if let Some(index) = self.tasks.iter().position(|task| task.name == task_name) {
            self.tasks.remove(index);
            return Ok(());
        }

        Err(TaskMgrError::TaskNotFound)
    }

    pub fn select(&self, query: &String) -> Result<Vec<Task>, TaskMgrError> {
        let query = query.trim();
        if query.is_empty() {
            return Err(TaskMgrError::WrongQuery);
        }

        if query == "select *" {
            return Ok(self.tasks.clone());
        }

        let query = query.replace("select * where", "");

        let parsed_args = parse_args(&query);
        if let Err(e) = parsed_args {
            return Err(TaskMgrError::WrongQueryPropExpr(e));
        }

        let parsed_args = parsed_args.unwrap();

        let mut temp = self.tasks.clone();

        for arg in parsed_args {
            temp = temp
                .iter()
                .filter(|task| {
                    self.match_field(&arg, task).is_ok() && self.match_field(&arg, task).unwrap()
                })
                .cloned()
                .collect();
        }

        Ok(temp)
    }

    fn match_field(
        &self,
        (leftvar, op, other): &(LeftVar, Op, String),
        task: &Task,
    ) -> Result<bool, TaskMgrError> {
        match leftvar {
            LeftVar::Name => Ok(self.compare_with_op(&task.name.to_string(), other, op)),
            LeftVar::Description => {
                Ok(self.compare_with_op(&task.description.to_string(), other, op))
            }
            LeftVar::Date => {
                let parsed_date = parse_task_date(other.clone());
                if let Err(e) = parsed_date {
                    return Err(TaskMgrError::GeneralTaskError(e));
                }
                return Ok(self.compare_with_op(&task.date, &parsed_date.unwrap(), op));
            }
            LeftVar::Category => Ok(self.compare_with_op(&task.category.to_string(), other, op)),
            LeftVar::Done => Ok(self.compare_with_op(&task.done.to_string(), other, op)),
        }
    }

    /// Because of the disruptive [NaiveDateTime] and [bool] in our [Task] struct fields
    /// we need a generic function to compare all of the involved types.
    /// Thankfully [NaiveDateTime] already implements [PartialEq] and [PartialOrd]
    /// so we can easily use comparison operators between them, but we also need `<T>`
    /// to implement [Display], since we're not converting [String] input to [bool], but instead comparing their `to_string()` values.
    ///
    /// ```assert_eq!(true.to_string(), "true");```
    fn compare_with_op<T>(&self, one: &T, other: &T, op: &Op) -> bool
    where
        T: PartialEq + PartialOrd + Display,
    {
        match op {
            Op::Equals => one.eq(other),
            Op::NotEquals => one.ne(other),
            Op::Greater => one.gt(other),
            Op::GrEquals => one.ge(other),
            Op::Less => one.lt(other),
            Op::LeEquals => one.le(other),
            Op::Like => one.to_string().contains(&other.to_string()),
        }
    }
}
