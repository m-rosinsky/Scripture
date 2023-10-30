mod lexer;
mod error;

pub fn run(input_f: &str) -> Result<(), error::ScrError> {
    let mut main_lexer = lexer::Lexer::new();
    
    let _ = main_lexer.tokenize_file(input_f)?;

    for token in main_lexer.table {
        println!("{:?}", token);
    }

    Ok(())
}