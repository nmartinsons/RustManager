use rusqlite::{Connection, Result};
use std::io;


fn connection() -> Result<()>{
    let _conn = Connection::open("RustData.db")?;
    println!("Connection established successfully!");
    Ok(())
}

struct Task {
    title: String,
    description: String,
    due_date: Date,
    priority: Int,
    status: String,
}


//ability to create tasks
fn create_data(){
    
}


//ability to view tasks
fn view_data(){
    
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


fn main(){
    if let Err(e) = connection() {
        eprintln!("Error establishing connection: {}", e);
    }

    loop {
        println!("\nOptions:");
        println!("1. Create task");
        println!("2. View tasks");
        println!("3. Update task");
        println!("4. Delete task");
        println!("5. Display incomplete tasks");
        println!("6. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => create_data(),
            2 => view_data(),
            3 => update_data(),
            4 => delete_data(),
            5 => view_incomplete_tasks(),
            6 => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
    
}
