extern crate rusqlite;
use rusqlite::{Connection, Result};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let conn = Connection::open("tasks.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            description TEXT,
            completed BOOLEAN
        )",
        [],
    )?;

    if args.len() == 1 {
        let tasks = list_tasks(&conn).unwrap();

        if tasks.len() == 0 {
            println!("Nothing to do :)");
        }

        for task in tasks {
            if task.2 {
                println!("\x1b[32m{} {}\x1b[0m", task.0, task.1);
            } else {
                println!("{} {}", task.0, task.1);
            }
        }
    } else {
        match args[1].as_str() {
            "add" => {
                let description = args[2..].join(" ");
                insert_task(&conn, &description).unwrap();
            }

            "list" => {
                let tasks = list_tasks(&conn).unwrap();

                if tasks.len() == 0 {
                    println!("Nothing to do :)");
                }

                for task in tasks {
                    if task.2 {
                        println!("\x1b[32m{} {}\x1b[0m", task.0, task.1);
                    } else {
                        println!("{} {}", task.0, task.1);
                    }
                }
            }

            "done" => {
                let task_id = args[2].parse::<i32>().unwrap();
                complete_task(&conn, task_id).unwrap();
            }

            "remove" => {
                let task_id: i32 = args[2].parse::<i32>().unwrap();
                remove_task(&conn, task_id).unwrap();
            }

            "clear" => {
                remove_all(&conn).unwrap();
            }

            "help" => {
                help();
            }

            _ => {
                eprintln!("No such command.");
                help();
            }
        }
    }

    Ok(())
}

fn insert_task(conn: &Connection, description: &String) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (description, completed) VALUES (?1, 0)",
        [description],
    )?;
    Ok(())
}

fn list_tasks(conn: &Connection) -> Result<Vec<(i32, String, bool)>> {
    let mut stmt = conn.prepare("SELECT id, description, completed FROM tasks")?;
    let task_iter = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;
    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task?);
    }
    Ok(tasks)
}

fn complete_task(conn: &Connection, task_id: i32) -> Result<()> {
    conn.execute("UPDATE tasks SET completed = 1 WHERE id = ?1", [task_id])?;
    Ok(())
}

fn remove_task(conn: &Connection, task_id: i32) -> Result<()> {
    conn.execute("DELETE FROM tasks WHERE id = ?1", [task_id])?;
    Ok(())
}

fn remove_all(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM tasks", [])?;
    Ok(())
}

fn help() {
    println!("Here is the list of all available commands:\n1 add - to add a new task.\n2 remove = to remove a task with a specific id.\n3 list - to get a list of all tasks along with their ids.\n4 done - to mark a task as done.\n5 clear - to clear the entire todo list.\n5 help - Details about the todo commandline tool.");
}
