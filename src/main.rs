mod back;
use back::*;
use errors::TaskMgrError;
use std::io::stdin;
use task_manager::TaskManager;

fn main() {
    let mut task_manager = TaskManager::new();
    loop {
        println!("\n\tBlazingly Fast Task Manager (glorified to-do list) v.00.00.032.1a\n");
        println!("- 'add %name%, %description%, %date%, %status%' creates a new task.");
        println!("- `update %task_name%` updates a task with new values.");
        println!("- 'delete %task_name%' removes a task.");
        println!("- `select *` lists all of the tasks.");
        println!("- `select * where` shows tasks filtered by the specified args.\n-\t- For example: 'select * where name=task and category=misc and status=true'");
        println!("-\t- Available args: 'name, description, date, category, status'\n");

        let input = manage_input(&mut task_manager);
        match input {
            Ok(ok) => println!("{}", ok),
            Err(e) => println!("Error: {:?}", e),
        }
        println!("Enter to continue...");
        stdin().read_line(&mut String::new()).unwrap();
    }
}

fn match_input(input: &String, task_manager: &mut TaskManager) -> Result<String, TaskMgrError> {
    task_manager.exec_command(input)
}

fn manage_input(task_manager: &mut TaskManager) -> Result<String, TaskMgrError> {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Input error.");

    let command_is_update = expression::command_equals(&input, "update").unwrap();
    if command_is_update {
        input = compose_update_query(&input)
    }

    let matched = match_input(&input, task_manager);
    if let Err(e) = matched {
        return Err(e);
    }

    Ok(matched.unwrap())
}

fn compose_update_query(update_task_name: &String) -> String {
    let (mut new_name, mut desc, mut date, mut cat) =
        (String::new(), String::new(), String::new(), String::new());

    println!("Enter new task name: ");
    stdin().read_line(&mut new_name).unwrap();

    println!("Enter new task description: ");
    stdin().read_line(&mut desc).unwrap();

    println!("Enter new task date: ");
    stdin().read_line(&mut date).unwrap();

    println!("Enter new task category: ");
    stdin().read_line(&mut cat).unwrap();

    format!(
        "{}; {}; {}; {}; {}",
        update_task_name,
        new_name.trim(),
        desc.trim(),
        date.trim(),
        cat.trim()
    )
}
