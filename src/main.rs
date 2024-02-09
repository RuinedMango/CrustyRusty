fn handle_arguments() {
    let mut args : Vec<_> = std::env::args().collect(); // get all arguements passed to app
    args.remove(0);
    println!("{:?}", args);
    if &args[0] == "cool" {
        print!("Cool found");
    }
}

fn main() {
    handle_arguments();
}
