use std::env::args;


struct Commands{
    function: String,
    key: String
}

fn main() {
    let function = args().nth(1).expect("Please provide action to perform");
    let key = args().nth(2).expect("Please key to perform");

    let args= Commands{
        function,
        key
    };

    println!("function is {:?} and key is {:?}",args.function, args.key );
}


fn input_handler(){
    println!("This will handle the input");
}

fn view(){
    println!("View alll the tasks");
}

fn add(){
    println!("Add a task");
}

fn done(){
    println!("Mark a task as done");
}

fn delete(){
    println!("delete a task");
}