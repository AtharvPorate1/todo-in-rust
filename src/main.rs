use std::env::args;
// use rusqlite::Connection;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};

struct Commands{
    function: String,
    key: String
}

// struct Todo{
//     id: u32,
//     task: String,
//     status: bool,
// }

 

fn main() {
    

   

    let function = args().nth(1).expect("Please provide action to perform");
    let key = args().nth(2).expect("Please key to perform");

    let args= Commands{
        function,
        key
    };

    println!("function is {:?} and key is {:?}",args.function, args.key );

    match args.function.as_str(){
        "add" => add(args.key),
        "done" => done(args.key),
        "delete" => delete(args.key),
        "view" => view(),
        _ => println!("Invalid command")
    }

}



fn view(){
    let file = File::open("tasks.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines(){
        let task = line.expect("Could not read line");
        println!("{}", task);
    }
}

fn add(task: String){
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("tasks.txt")
        .expect("Could not open file");

    let last_index: = file.read(buf) 

    file.write_all(format!("\nt- ){}", task).as_bytes())
        .expect("Failed writing to the file");
}

fn done(key: String){
    println!("Mark a task as done {} ", key);
}

fn delete(key: String){
    println!("delete a task {} ", key);
}