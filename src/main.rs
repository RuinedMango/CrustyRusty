fn handle_arguments() {
    let mut args : Vec<_> = std::env::args().collect(); // get all arguements passed to app
    args.remove(0);
    if &args[0] == "cool" {
        print!("Cool found"); 
        let mut cmd_args : Vec<String> = Vec::new();
        cmd_args.push(args[1].clone());
        cmd_args.push(args[2].clone());
        cmd_args.push(args[3].clone());
        cool(cmd_args);
    }
}

fn main() {
    handle_arguments();
}

fn cool(args: Vec<String>){
    println!("{:?}", args[0]);
    println!("{:?}", args[1]);
    println!("{:?}", args[2]);
}