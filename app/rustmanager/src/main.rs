use rusqlite::{Connection, Result};
use std::io::{self, Write};


fn connection() -> Result<Connection>{
    let _conn = Connection::open("RustData.db")?;
    println!("Connection established successfully!");
    Ok(_conn)
}

struct Task {
    title: String,
    description: String,
    due_date: String,
    priority: i32,
    status: String,
}


//ability to create tasks
fn create_data(conn: &Connection, task: &Task) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (title, description, due_date, priority, status) VALUES (?1, ?2, ?3, ?4, ?5)",
        &[&task.title, &task.description, &task.due_date, &task.priority.to_string(), &task.status],
    )?;

    println!("\nTask created successfully!");
    Ok(())
}


//ability to view tasks
fn view_data(conn: &Connection) -> Result<()>{
    let mut stmt = conn.prepare("SELECT title, description, due_date, priority, status FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<usize, String>(0)?,
            row.get::<usize, String>(1)?,
            row.get::<usize, String>(2)?,
            row.get::<usize, i32>(3)?,
            row.get::<usize, String>(4)?,
        ))
    })?;

    println!("\nTasks:\n");
    for task in task_iter {
        if let Ok((title, description, due_date, priority, status)) = task {
            print!("Title: {}", title);
            print!("Description: {}", description);
            print!("Due Date: {}", due_date);
            print!("Priority: {}", priority);
            print!("\nStatus: {}\n", status);
        }
    }

    Ok(())
    
}


//ability to update data
//ability to update data
fn update_data(conn: &Connection) -> Result<()> {
    
}




//ability to delete data
fn delete_data(conn: &Connection) -> Result<()> {
    println!("Enter the title of the task to delete: ");
    let mut input_title = String::new();
    let _ = io::stdin().read_line(&mut input_title);

    // Trim the input_title to remove leading/trailing whitespaces
    let input_title = input_title.trim();

    // Delete the task
    conn.execute("DELETE FROM tasks WHERE title LIKE ?1", &[&format!("%{}%", input_title)])?;
    println!("Task with title '{}' deleted successfully!", input_title);

    Ok(())
}



//ablity to see number of incomplete tasks
fn view_incomplete_tasks(conn: &Connection) -> Result<()>{
    let mut stmt = conn.prepare("SELECT title, description, due_date, priority, status FROM tasks WHERE status LIKE '%incomplete%'")?;
    let task_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<usize, String>(0)?,
            row.get::<usize, String>(1)?,
            row.get::<usize, String>(2)?,
            row.get::<usize, i32>(3)?,
            row.get::<usize, String>(4)?,
        ))
    })?;

    println!("\nTasks:\n");
    for task in task_iter {
        if let Ok((title, description, due_date, priority, status)) = task {
            print!("Title: {}", title);
            print!("Description: {}", description);
            print!("Due Date: {}", due_date);
            print!("Priority: {}", priority);
            print!("\nStatus: {}\n", status);
        }
    }

    Ok(())

}


fn main()-> Result<()>{
    let conn = connection()?;

    loop {
        println!("\nOptions:");
        println!("1. Create task");
        println!("2. View tasks");
        println!("3. Update task");
        println!("4. Delete task");
        println!("5. Display incomplete tasks");
        println!("6. Exit");

        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        let choice: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                let mut task = Task {
                    title: String::new(),
                    description: String::new(),
                    due_date: String::new(),
                    priority: 0,
                    status: String::new(),
                };

                println!("\nEnter task details:");

                print!("Title: ");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut task.title);

                print!("Description: ");
                let _  = io::stdout().flush();
                let _ = io::stdin().read_line(&mut task.description);

                print!("Due Date: ");
                let _  = io::stdout().flush();
                let _ = io::stdin().read_line(&mut task.due_date);

                print!("Priority: ");
                let _  = io::stdout().flush();
                let mut priority = String::new();
                let _ = io::stdin().read_line(&mut priority);
                task.priority = priority.trim().parse().unwrap_or(0);

                print!("Status: ");
                let _  = io::stdout().flush();
                let _ = io::stdin().read_line(&mut task.status);


                create_data(&conn, &task)?;
            }
            2 => view_data(&conn)?,
            3 => update_data(&conn)?,
            4 => delete_data(&conn)?,
            5 => view_incomplete_tasks(&conn)?,
            6 => {
                println!("Exiting program.");
                break Ok(());
            }
            _ => println!("Invalid choice. Please try again."),
        }

    }
}
