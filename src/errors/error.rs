use colored::*;


#[derive(Debug)]
pub enum ETypes {
    SyntaxError,
    DivideByZeroError,
    FileError,
}
pub struct GError {
    etype : ETypes,
    message : String,
    file : String,
    linecontents : String,
    line : usize,
    col : usize,
}
impl GError {
    pub fn new(etype : ETypes, message : &str, file : String, linecontents : String, line : usize, col : usize) -> GError {
        GError {
            etype, message : message.to_string(), file, linecontents, line, col 
        }
    }
}
pub struct ErrorStack {
    pub errors : Vec<GError>,
}
impl ErrorStack {
    pub fn new() -> ErrorStack {
        ErrorStack {
            errors : Vec::new()
        }
    }
    pub fn print_dump(&self) {
        for error in &self.errors {
            println!(
                "{}{}{}{}{}{}: \n  {:?}: {} \n  --> {} ",
                "gscript '".red(),
                error.file.yellow(), 
                "': Line ".red(),
                error.line.to_string().red(), 
                ", Char ".red(),
                error.col, error.etype, error.message.red(), error.linecontents.red()
            );
            for _i in 0..error.col {
                print!(" ");
            }
            print!("^");
            
        }
    } 
}
