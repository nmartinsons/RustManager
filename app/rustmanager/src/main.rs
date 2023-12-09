use rusqlite::{Connection, Result};
use std::io;


fn connection() -> Result<()>{
    let _conn = Connection::open("RustData.db")?;
    println!("Connection established successfully!");
    Ok(())
}


//ability to create tasks
fn createData(){
    
}


//ability to view tasks
fn viewData(){
    
}


//ability to update data
fn updateData(){
    
}


//ability to delete data
fn deleteData(){
    
}


//ablity to see number of incomplete tasks
fn viewIncompleteTasks(){

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
            1 => createData(),
            2 => viewData(),
            3 => updateData(),
            4 => deleteData(),
            5 => viewIncompleteTasks(),
            6 => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
    
}
