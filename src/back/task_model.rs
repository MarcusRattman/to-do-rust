#![allow(dead_code)]
use super::TaskError;
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub date: NaiveDateTime,
    pub category: String,
    pub done: bool,
}

impl Task {
    pub fn new(task: &str) -> Result<Self, TaskError> {
        let task: Vec<String> = task
            .split(",")
            .map(|task| task.trim().to_string())
            .collect();

        if task.len() != 4 {
            return Err(TaskError::TaskCreationArgsError);
        }

        let parsed = parse_task_date(task[2].to_string())?;

        Ok(Task {
            name: task[0].to_string(),
            description: task[1].to_string(),
            date: parsed,
            category: task[3].to_string(),
            done: false,
        })
    }

    pub fn check_done(&mut self) {
        self.done = !self.done;
    }

    pub fn update(
        &mut self,
        name: &str,
        description: &str,
        date: &str,
        cat: &str,
    ) -> Result<&Self, TaskError> {
        let parsed = parse_task_date(date.to_string())?;
        self.name = name.to_string();
        self.description = description.to_string();
        self.date = parsed;
        self.category = cat.to_string();

        Ok(self)
    }
}

fn parse_task_date(date: String) -> Result<NaiveDateTime, TaskError> {
    let parsed = NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M");
    if parsed.is_err() {
        return Err(TaskError::TaskDateParseError);
    }

    return Ok(parsed.unwrap());
}
