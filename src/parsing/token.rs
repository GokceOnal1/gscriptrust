pub enum Token {
    INT(i32),
    FLOAT(f32),
    STRING(String)
}
impl Token {
    pub fn print_toks(toks : &Vec<Token>) {
        for t in toks {
            match t {
                Token::STRING(s) => println!("STRING: {s}"),
                Token::FLOAT(f) => println!("FLOAT: {f}"),
                Token::INT(i) => println!("INT: {i}"),
                //_ => println!("UNPRINTABLE TOKEN"),
            }
        }
    }
}
