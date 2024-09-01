use colored::*;


#[derive(Debug)]
pub enum ETypes {
    SyntaxError,
    DivideByZeroError,
    FileError,
    TokenError,
    EndOfInputError,
    VariableDefinitionError,
    FunctionDefinitionError,
    FunctionError,
    ConditionalError,
    TypeError,
    ListError,
    BlueprintError,
    IdentifierError
}
impl std::fmt::Display for ETypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::SyntaxError => write!(f, "SyntaxError"),
            Self::DivideByZeroError => write!(f, "DivideByZeroError"),
            Self::FileError => write!(f, "FileError"),
            Self::TokenError => write!(f, "TokenError"),
            Self::EndOfInputError => write!(f, "EndOfInputError"),
            Self::VariableDefinitionError => write!(f, "VariableDefinitionError"),
            Self::FunctionDefinitionError => write!(f, "FunctionDefinitionError"),
            Self::FunctionError => write!(f, "FunctionError"),
            Self::ConditionalError => write!(f, "ConditionalError"),
            Self::TypeError => write!(f, "TypeError"),
            Self::ListError => write!(f, "ListError"),
            Self::BlueprintError => write!(f, "BlueprintError"),
            Self::IdentifierError => write!(f, "IdentifierError")
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
#[derive(Clone, Debug)]
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
    pub fn set_endln(&mut self) {
        self.col = self.linecontents.len();
        self.col_end = self.col + 1;
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
    pub fn warn(&self, einfo : ErrorInfo, warning : &str) {
        eprintln!("{}{}{}{}{}\n{}{}\n  --> {}",
            "in file ".yellow(),
            einfo.file.yellow(),
            ", at line ".yellow(),
            einfo.line.to_string().yellow(),
            ":".yellow(),
            "warning: ".yellow().bold(),
            warning.yellow(),
            einfo.linecontents
        );
        for _i in 0..einfo.col+5 {
            eprint!(" ");
        }
        for _i in 0..einfo.col_end-einfo.col {
            eprint!("{}","^".yellow().bold());
        }
        eprintln!();
    }
    pub fn print_dump(&self) {
        if self.errors.len() != 0 {
            eprintln!("{}","-----------------------------------------".red());
        }
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
