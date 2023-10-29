mod lexer;

pub fn run(input_f: &str) {
    let mut main_lexer = lexer::Lexer::new();
    
    if let Err(err) = main_lexer.tokenize_file(input_f) {
        eprintln!("Error: {}", err);
        return;
    }

    for token in main_lexer.table {
        println!("{:?}", token);
    }
}