use circle_argument::game_circle::test_model_circle;
use console;
use std::io::{self, Write};
use crate::{circle_argument::game_circle::{ interact_model_circle }, overall_situation::overall_variables};
use crate::test_solver::average_times::test_average_guess_times;

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
mod test_solver;

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = atty::is(atty::Stream::Stdout);
    //let is_tty: bool = false;
    let mut if_c: bool = false;

    if is_tty {

        let mut if_world_solver: bool = false;
        // start the wordle-solver, the argument is --solver
        let mut if_need_tips: bool = false;
        // give all the possible guesses, the argument is --possible
        for arg in std::env::args() {
            if arg == "--solver".to_string() {
                if_world_solver = true;
            }
            if arg == "--possible".to_string() { // give all the possible words
                if_need_tips = true;
            }
            if arg == "--rec".to_string() { // give the recommended words
                unsafe { overall_variables::NEED_RECOMMEND = true; }
            }
            if arg == "--test".to_string() {
                unsafe { overall_variables::IF_CALCULATE = true; }
                if_c = true;
            }
            if arg == "--first" { // give the best start-words
                unsafe { test_solver::average_times::IF_FIRST = true; }
            }
        }
        
        if if_c == false {

            print!("{}", console::style("Your name: ").bold().red());
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin().read_line(&mut line)?;
            println!("Welcome to wordle, {}!", 
                console::style(line.trim()).blink().red());

            if if_world_solver == true {
                wordle_solver::solver::start_solver();
            } else {
                if if_need_tips == true {
                    unsafe { overall_situation::
                    overall_variables::NEED_TIP = true; }
                }
                interact_model_circle();
            }
        } else {
            //println!("break 2");
            test_average_guess_times();
        }
        
    } else {
        test_model_circle();
    }

    Ok(())
}
