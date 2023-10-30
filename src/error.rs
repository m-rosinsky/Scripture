use colored::*;

pub enum ScrErrorType {
    Success,

    // File errors.
    FileNotFound,
    FileReadError,
}

fn err_to_string(err: &ScrErrorType) -> String {
    match err {
        ScrErrorType::Success => String::from("Success"),
        ScrErrorType::FileNotFound =>
            String::from("Thine scripture is unbeknownst to the heavens"),
        ScrErrorType::FileReadError =>
            String::from("Thine scripture harbors forbidden symbols"),
    }
}

pub struct ScrError {
    /// The error type.
    pub err_type: ScrErrorType,
    /// The file name the error occurred in.
    pub fname: String,
    /// The line number the error occurred on.
    pub line_num: u32,
    /// The column number the error occurred on.
    pub col_num: u32,
    /// The optional message to display.
    pub msg: String,
}

impl ScrError {
    pub fn report(&self) {
        eprint!("{}", "Blasphemy".red());

        if self.line_num != 0 {
            eprint!("[{} {}:{}]",
                self.fname,
                self.line_num,
                self.col_num,
            );
        }
        else {
            eprint!(": ");
        }

        eprint!("{}", err_to_string(&self.err_type));

        if !self.msg.is_empty() {
            eprintln!(": '{}'", self.msg);
        }
        else {
            eprintln!("");
        }
    }
}