fn error(text &String){
    println!("Fatal Error: {}", text);
    std::process::exit(1);
}
