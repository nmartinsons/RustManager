use rusqlite::{Connection, Result};

fn main(){
    if let Err(e) = connection() {
        eprintln!("Error establishing connection: {}", e);
    }
    
}

fn connection() -> Result<()>{
    let _conn = Connection::open("RustData.db")?;
    println!("Connection established successfully!");
    Ok(())
}