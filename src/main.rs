mod lexer;
use lazy_static::lazy_static;
fn main() {
    // remember that the max size is 4096 bytes!!
    // No more stupid little 'a annotations. yay!!
    lazy_static! {
        static ref FILE: String = {
            match std::env::args().nth(1) {
                Some(filename) => {
                    match std::fs::read_to_string(filename) {
                        Ok(file) => file.to_ascii_lowercase(),
                        Err(_) => { eprintln!("Cannot read file"); std::process::exit(1); },
                    }
                },
                None => { eprintln!("Please enter a file"); std::process::exit(1); },
            }
        };
    }

    let lexer = lexer::Lexer::new(&FILE);

    lexer.lexer();
}
