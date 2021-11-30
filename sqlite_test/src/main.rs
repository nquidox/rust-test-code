use rusqlite::*;


struct Command{
    name: String,
    content: String
}


fn connect() -> Connection{
    let db_connection = Connection::open("test.db")
        .expect("Database file not found.");
    return db_connection;
}


fn create_table(db: Connection, table_name: String){
    let sql = format!("CREATE TABLE IF NOT EXISTS {} (command TEXT, content TEXT)", table_name);
    db.execute(&*sql, [])
        .expect("SQL query failed.");
}

#[allow(dead_code)]
fn write_to_table(db: Connection, table_name: String, record: Command){
    let sql = format!("INSERT INTO {} (command, content) VALUES (?1, ?2)", table_name);
    db.execute(&*sql, params![record.name, record.content]).expect("Error");
}


fn read_from_table(db: Connection, table_name: String) -> Result<()>{
    let sql = format!("SELECT * FROM {}", table_name);
    let mut stmt = db.prepare(&*sql)?;
    let commands_iter = stmt.query_map([], |row|{
        Ok(Command{name: row.get(0)?, content: row.get(1)?})
    })?;

    for command in commands_iter{
        println!("{:?} : {:?}", command.as_ref().unwrap().name, command.as_ref().unwrap().content);
    }
    Ok(())
}


fn main(){
    let db = connect();
    let table_name = String::from("Kuraj");
    create_table(db, table_name);

    // write fn
    let db = connect();
    let table_name = String::from("Kuraj");
    let record = Command{
        name: "duck".to_string(),
        content: "This duck is happy. Someone told him he's cute.".to_string()
    };
    write_to_table(db, table_name, record);

    //read fn
    let db = connect();
    let table_name = String::from("Kuraj");
    read_from_table(db, table_name);

}
