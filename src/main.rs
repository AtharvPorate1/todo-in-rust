use std::env::args;
use rusqlite::{params, Connection};
 
// Next : Use tUI to make it more interactive
fn main() {

    let conn = Connection::open("file.db").expect("Failed to open Database");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Todo (
            id    INTEGER PRIMARY KEY,
            task  TEXT NOT NULL,
            status  BOOl
        )",
        (), // empty list of parameters.
    ).expect("Failed to execute query");
    
    let args: Vec<String> = args().collect();
    
    if args.len()>1{
        let command = &args[1];

        match &command[..]{
            "list" => view(),
            "add"  => add(&args[2..]),
            "done" => done(&args[2..]),
            "delete" => delete(&args[2..]),
            _ => println!("Invalid command")
        }
    }else{
        view();

    }
    
   

    

}



fn view(){
    let conn = Connection::open("file.db").expect("Failed to open Database");

    let mut stmt = conn.prepare("SELECT id, task, status FROM Todo").expect("Failed to prepare query");
    let todo_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, bool>(2)?,
        ))
    }).expect("Failed to execute query");

    for todo in todo_iter {
        let (id, task, status) = todo.expect("Failed to unwrap todo");
        if status {
            println!("{}: \x1B[9m{}\x1B[0m", id, task);
        } else {
            println!("{}: {}", id, task);
        }
    }
}

fn add( args: &[String]){
    let conn = Connection::open("file.db").expect("Failed to open Database");

    for data in args{
        let task = data;
        conn.execute(
            "INSERT INTO Todo (task, status) VALUES (?1, ?2)",
            (&task, &false),
        ).expect("Failed to execute query");
    }

    
    
}

fn done(args: &[String]){
    let conn = Connection::open("file.db").expect("Failed to open Database");
    for task in args{
        conn.execute(
            "UPDATE Todo SET status = ?1 WHERE id = ?2",
            (&true, &task),
        ).expect("Failed to execute query");
    }
}
fn delete(args: &[String]) {
    let conn = Connection::open("file.db").expect("Failed to open Database");

    for id in args {
        let id_int: i32 = id.parse().expect("Failed to parse id");
        conn.execute(
            "DELETE FROM Todo WHERE id = ?1",
            params![id_int],
        ).expect("Failed to execute query");
    }
}