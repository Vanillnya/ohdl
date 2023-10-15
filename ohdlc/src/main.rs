use lexer::Lexer;

mod lexer;
mod token;
mod util;

fn main() {
    let text = include_str!("work.ohd");
    let mut lexer = Lexer::new(text);
    println!("{:?}", lexer.next());
    println!("{:?}", lexer.next());
    println!("{:?}", lexer.next());
}
