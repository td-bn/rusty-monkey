use rusty_monkey::repl;

fn main() {
    println!();
    println!("Hello, world!");
    println!("Welcome to rusty-monkey. A rusty flavour of the Monkey programming language.");
    println!();
    println!("Press <Ctrl> + C to exit the interpretor at any time");
    repl::start();
}
