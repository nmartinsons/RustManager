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
    
}
