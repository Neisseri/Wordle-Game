use circle_argument::game_circle::test_model_circle;
use console;
use std::io::{self, Write};
use crate::circle_argument::game_circle::{ interact_model_circle };

mod test_model;
mod builtin_words;
mod interact_model;
mod my_tool;
mod circle_argument;
mod overall_situation;
mod parse_json;
mod config;
mod possible_words;
mod wordle_solver;

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = atty::is(atty::Stream::Stdout);
    //let is_tty: bool = false;

    if is_tty {
        
        print!("{}", console::style("Your name: ").bold().red());
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        println!("Welcome to wordle, {}!", 
            console::style(line.trim()).blink().red());

        /*print!("Command line arguments: ");
        for arg in std::env::args() {
            print!("{} ", arg);
        }
        println!("");*/

        println!("{}",
            console::style("Do you want to start the WordleSolver? Y/N")
            .blink().on_blue());
        let if_solver: String = text_io::read!();
        if if_solver == "Y" || if_solver == "y" {
            wordle_solver::solver::start_solver();
        } else {

            println!("{}",
            console::style("Do you need some tips? Y/N")
            .blink().on_green());
            let if_tip: String = text_io::read!();
            if if_tip == "Y" || if_tip == "y" {
                unsafe { overall_situation::
                overall_variables::NEED_TIP = true; }
            }

            interact_model_circle();

        }

    } else {
        test_model_circle();
    }

    Ok(())
}
