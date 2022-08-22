use text_io::read;

use crate::my_tool::tool;

pub mod run_test_model {

    use super::tool::{match_words, Color, valid, args_parse};
    use crate::builtin_words;
    use rand::Rng;

    pub fn test_run() -> () {

        let mut is_word: Option<String> = None;
        let mut is_random: bool = false;
        (is_word, is_random) = args_parse();

        let mut guess_right: bool = true;
        let mut answer: String = String::new();

        if is_random == true { // the random model: read the answer from FINAL

            let mut rng = rand::thread_rng();
            let mut index: usize = rng.gen_range(0 ..= 2314);
            answer = builtin_words::FINAL[index].to_string();

        } else if is_word.is_some() == true { // read the answer from args
            
            answer = is_word.unwrap().to_lowercase();

        } else { // the basic model

            answer = text_io::read!();

        } // read answer from stdin
        
        let mut ans: Vec<char> = Vec::new();
        for words in answer.chars() {
            ans.push(words);
        } // read the Final words and convert into vector

        let mut count: Vec<i32> = vec![0; 26];
        for i in 0 ..= 4 {
            count[match_words(ans[i])] += 1;
        }

        let mut guess: String = String::new();
        let mut keyboard: Vec<Color> = vec![Color::Unknown; 26];
        
        for number in 1 ..= 6 {
            guess_right = true;

            guess.clear();
            std::io::stdin().read_line(&mut guess);
            guess.pop();
            //println!("{}", guess);
            while valid(&guess) == false {
                println!("INVALID");
                guess.clear();
                std::io::stdin().read_line(&mut guess);
                guess.pop();
            }

            let mut gus: Vec<char> = Vec::new();
            for words in guess.chars() {
                gus.push(words);
            } // read the input words and convert into chars

            let mut colors: Vec<Color> = vec![Color::Unknown; 5];//the color of XXXXX

            let mut cnt_gus: Vec<i32> = vec![0; 26];
            // the number of letters in the guessed word

            for i in 0 ..= 4 {
                let letter_num = match_words(gus[i]);
                cnt_gus[letter_num] += 1;
                //count the number of used letters

                if gus[i] == ans[i] {
                    colors[i] = Color::Green;
                } else {

                    guess_right = false;

                    if cnt_gus[match_words(gus[i])] <= count[match_words(gus[i])]
                    {
                        colors[i] = Color::Yellow;
                    } else {
                        colors[i] = Color::Red;
                    }
                } // give the color of XXXXX

                if colors[i] == Color::Red {
                    if keyboard[match_words(gus[i])] == Color::Unknown {
                        keyboard[match_words(gus[i])] = Color::Red;
                    }
                }
                if colors[i] == Color::Yellow {
                    if keyboard[match_words(gus[i])] == Color::Red {
                        keyboard[match_words(gus[i])] = Color::Yellow;
                    }
                    if keyboard[match_words(gus[i])] == Color::Unknown {
                        keyboard[match_words(gus[i])] = Color::Yellow;
                    }
                }
                if colors[i] == Color::Green {
                    if keyboard[match_words(gus[i])] == Color::Yellow {
                        keyboard[match_words(gus[i])] = Color::Green;
                    } 
                    if keyboard[match_words(gus[i])] == Color::Red {
                        keyboard[match_words(gus[i])] = Color::Green;
                    }
                    if keyboard[match_words(gus[i])] == Color::Unknown {
                        keyboard[match_words(gus[i])] = Color::Green;
                    }
                }

            } //process the word

            for i in 0 ..= 4 {
                match colors[i] {
                    Color::Green => print!("G"),
                    Color::Red => print!("R"),
                    Color::Yellow => print!("Y"),
                    _ => ()
                }
            }

            print!(" ");
            for i in 0 ..= 25 {
                match keyboard[i] {
                    Color::Green => print!("G"),
                    Color::Red => print!("R"),
                    Color::Yellow => print!("Y"),
                    Color::Unknown => print!("X")
                }
            }
            println!(""); // print the keyboard

            if guess_right == true {
                println!("CORRECT {}", number);
                break;
            }
        }

        if guess_right == false {
            println!("FAILED {}", answer.to_uppercase());
        }

    }
}