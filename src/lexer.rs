pub struct Lexer {
    data : &'static str
}

impl Lexer {
    pub fn new(data: &'static str) -> Self { Self { data } }

    pub fn lexer(&self) {
        self.data.chars().for_each(|x| println!("{}", x));
    }
}
