use console;
use std::io::{self, Write};
use crate::test_model::run_test_model;
use crate::interact_model::run_interact_model;

mod test_model;
mod builtin_words;
mod interact_model;
mod my_tool;

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = atty::is(atty::Stream::Stdout);
    //let is_tty: bool = false;

    /*if is_tty {
        println!(
            "I am in a tty. Please print {}!",
            console::style("colorful characters").bold().blink().blue()
        );
    } else {
        println!("I am not in a tty. Please print according to test requirements!");
    }*/

    if is_tty {
        print!("{}", console::style("Your name: ").bold().red());
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        println!("Welcome to wordle, {}!", line.trim());

        // example: print arguments
        print!("Command line arguments: ");
        for arg in std::env::args() {
            print!("{} ", arg);
        }
        println!("");
        // TODO: parse the arguments in `args`

        run_interact_model::interact_run();

    } else {
        run_test_model::test_run();
    }

    Ok(())
}
