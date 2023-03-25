pub fn error(text: &str){
    println!("Fatal Error: {}", text);
    std::process::exit(1);
}
