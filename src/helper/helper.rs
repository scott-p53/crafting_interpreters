#[derive(Debug, Clone)]
pub struct Error {
    reason: String,
    line: u128,
}

impl Error {
    pub fn new(line: u128, reason: String) -> Self {
        return Error { line, reason };
    }
}

pub fn report_errors(errors: &Vec<Error>) {
    for error in errors {
        println!("[Line {} ] Error: {}", error.line, error.reason);
    }
}
