#[derive(Debug)]
pub struct TeciError {
    line: usize,
    message: String,
}

impl TeciError {
    pub fn new(line: usize, message: String) -> Self {
        TeciError { line, message }
    }

    pub fn report(&self, loc: String) {
        eprintln!("[line {}] Error {}: {}", self.line, self.message, loc);
    }
}
