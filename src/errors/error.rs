use colored::*;


#[derive(Debug)]
pub enum ETypes {
    SyntaxError,
    DivideByZeroError,
    FileError,
    TokenError,
    EndOfInputError,
}
impl std::fmt::Display for ETypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::SyntaxError => write!(f, "SyntaxError"),
            Self::DivideByZeroError => write!(f, "DivideByZeroError"),
            Self::FileError => write!(f, "FileError"),
            Self::TokenError => write!(f, "TokenError"),
            Self::EndOfInputError => write!(f, "EndOfInputError"),
        }
    }
}
pub struct GError {
    etype : ETypes,
    message : String,
    file : String,
    linecontents : String,
    line : usize,
    col : usize,
    col_end : usize,
}
impl GError {
    pub fn new(etype : ETypes, message : &str, file : String, linecontents : String, line : usize, col : usize, col_end : usize) -> GError {
        GError {
            etype, message : message.to_string(), file, linecontents, line, col, col_end,
        }
    }
    pub fn new_from_tok(etype : ETypes, message : &str, einfo : ErrorInfo) -> GError {
        GError {
            etype, message : message.to_string(), file : einfo.file, linecontents : einfo.linecontents, line : einfo.line, col : einfo.col, col_end : einfo.col_end 
        }
    }
}
#[derive(Clone)]
pub struct ErrorInfo {
    pub file : String,
    pub linecontents : String,
    pub line : usize,
    pub col : usize,
    pub col_end : usize
}
impl ErrorInfo {
    pub fn new(file : String, linecontents : String, line : usize, col : usize, col_end : usize) -> ErrorInfo {
        ErrorInfo {file, linecontents, line, col, col_end }
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
        eprintln!("{}","-----------------------------------------".red());
        for error in &self.errors {
            eprintln!(
                "{}{}{}{}{}{} \n  {}{} {} \n  --> {} ",
                "gscript ".red().italic(),
                error.file.truecolor(186, 149, 48), 
                ": Line ".red(),
                error.line.to_string().red(), 
                ", Char ".red(),
                error.col.to_string().red(), error.etype.to_string().red().bold(), ":".red().bold(), error.message.red(), error.linecontents.bold()
            );
            for _i in 0..error.col+5 {
                eprint!(" ");
            }
            for _i in 0..error.col_end-error.col {
                eprint!("{}","^".red().bold());
            }
            eprintln!();
            eprintln!("{}","-----------------------------------------".red());
        }
    } 
    pub fn terminate_gs(&self) {
        self.print_dump();
        std::process::exit(1);
    }
}
