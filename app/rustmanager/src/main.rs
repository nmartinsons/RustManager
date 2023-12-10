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

    println!("Task created successfully!");
    Ok(())
}


//ability to view tasks
fn view_data(){
    println!("Tasks:");
    
}


//ability to update data
fn update_data(){
    
}


//ability to delete data
fn delete_data(){
    
}


//ablity to see number of incomplete tasks
fn view_incomplete_tasks(){

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
        io::stdin().read_line(&mut input);
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

                println!("Enter task details:");

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
            2 => view_data(),
            3 => update_data(),
            4 => delete_data(),
            5 => view_incomplete_tasks(),
            6 => {
                println!("Exiting program.");
                break Ok(());
            }
            _ => println!("Invalid choice. Please try again."),
        }

    }
}
