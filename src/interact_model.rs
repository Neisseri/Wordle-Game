use text_io::read;

pub mod run_interact_model {
    use core::num;

    use crate::{my_tool::{tool::{self, match_number, difficult_record}, self}, builtin_words::FINAL};

    pub fn interact_run() -> (bool, bool, bool) {

        use crate::my_tool::tool::{ match_words, Color, valid, args_parse };
        use console;
        use rand::Rng;
        use crate::builtin_words;
        use crate::overall_situation::overall_variables;

        let mut is_word: Option<String> = None;
        let mut is_random: bool = false;
        let mut is_difficult: bool = false;
        let mut is_stats: bool = false;
        (is_word, is_random, is_difficult, is_stats) = args_parse();

        let mut guess_right: bool = true;
        let mut answer: String = String::new();

        if is_random == true { // the random model: read the answer from FINAL

            let mut rng = rand::thread_rng();
            let mut index: usize = rng.gen_range(0 ..= 2314);
            unsafe {
                let mut if_repeat = overall_variables::word_history.iter().position(|r| r == &index);
                while if_repeat.is_some() == true {
                    index = rng.gen_range(0 ..= 2314);
                    if_repeat = overall_variables::word_history.iter().position(|r| r == &index);
                }
                overall_variables::word_history.push(index);
            }
            answer = FINAL[index].to_string();

        } else if is_word.is_some() == true { // read the answer from args

            answer = is_word.clone().unwrap().to_lowercase();

        } else { // the basic model

            // read in answer from stdin
            println!("Please type in the Answer!");
            
            std::io::stdin().read_line(&mut answer);
            answer.pop();
            while valid(&answer) == false {
                println!("{}", console::style("The word is invalid!").bold().red());
                answer.clear();
                std::io::stdin().read_line(&mut answer);
                answer.pop();
            }// read in answer from stdin

        }

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

        println!("You have 6 chances to guess the word!");

        let mut all_colors: Vec<Vec<Color>> = Vec::new();
        let mut all_keybd: Vec<Vec<Color>> = Vec::new();
        let mut all_guess: Vec<Vec<char>> = Vec::new();

        let mut dif_rec: Vec<difficult_record> = Vec::new();
        
        for number in 1 ..= 6 {

            println!("This is round {}:", number);

            guess_right = true;

            guess.clear();
            std::io::stdin().read_line(&mut guess);
            guess.pop();
            
            while valid(&guess) == false {
                println!("{}", console::style("The word is invalid!").bold().red());
                guess.clear();
                std::io::stdin().read_line(&mut guess);
                guess.pop();
            }

            let mut gus: Vec<char> = Vec::new();
            for words in guess.chars() {
                gus.push(words);
            } // read the input words and convert into chars

            if is_difficult == true && number > 1 {

                while tool::difficult_valid(&gus, &dif_rec) == false {

                    println!("{}",
                        console::style("The word is invalid in difficlut model!")
                        .bold().red()
                    );
                    guess.clear();
                    std::io::stdin().read_line(&mut guess);
                    guess.pop();

                    while valid(&guess) == false {
                        println!("{}", console::style("The word is invalid!").bold().red());
                        guess.clear();
                        std::io::stdin().read_line(&mut guess);
                        guess.pop();
                    }

                    gus.clear();
                    for words in guess.chars() {
                        gus.push(words);
                    } // read the input words and convert into chars
                }

            }

            all_guess.push(gus.clone());

            overall_variables::record_use_times(guess.clone());

            let mut colors: Vec<Color> = vec![Color::Unknown; 5];//the color of XXXXX

            let mut cnt_gus: Vec<i32> = vec![0; 26];
            // the number of letters in the guessed word

            dif_rec.clear();

            for i in 0 ..= 4 {
                if gus[i] == ans[i] {
                    cnt_gus[match_words(gus[i])] += 1;
                }
            }

            for i in 0 ..= 4 {
                
                if gus[i] == ans[i] {
                    colors[i] = Color::Green;
                } else {

                    let letter_num = match_words(gus[i]);
                    cnt_gus[letter_num] += 1;
                    //count the number of used letters

                    guess_right = false;

                    if cnt_gus[match_words(gus[i])] <= count[match_words(gus[i])]
                    {
                        colors[i] = Color::Yellow;
                    } else {
                        colors[i] = Color::Red;
                    }
                } // give the color of XXXXX

                dif_rec.push(difficult_record{
                    letter: match_words(gus[i]) as i32,
                    color: colors[i].clone()
                });

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

            all_colors.push(colors.clone());
            all_keybd.push(keyboard.clone());

            for j in 0 ..= number - 1 {

                for k in 0 ..= 4 {
                    match all_colors[j][k] {
                        Color::Green => print!("{}",console::style(all_guess[j][k].to_uppercase()).bold().green()),
                        Color::Red => print!("{}", console::style(all_guess[j][k].to_uppercase()).bold().red()),
                        Color::Yellow => print!("{}", console::style(all_guess[j][k].to_uppercase()).bold().yellow()),
                        _ => ()
                    }
                }

                print!(" ");
                for k in 0 ..= 25 {
                    match all_keybd[j][k] {
                        Color::Green => print!("{}",console::style(match_number(k)).bold().green()),
                        Color::Red => print!("{}", console::style(match_number(k)).bold().red()),
                        Color::Yellow => print!("{}", console::style(match_number(k)).bold().yellow()),
                        Color::Unknown => print!("{}", match_number(k))
                    }

                }
                println!(""); // print the keyboard

            }

            if guess_right == true {
                println!("You used {} chances and get the answer!", number);
                unsafe {
                    overall_variables::try_times.push(number as i32);
                }
                break;
            }
        }

        if guess_right == false {
            unsafe { overall_variables::fail_num += 1; }
            println!("You failed! The answer is {}!", answer.to_uppercase());
        } else {
            unsafe { overall_variables::success_num += 1; }
        }

        let mut word: bool = false;
        if is_word.is_some() == true {
            word = true;
        }
        (word, is_random, is_stats)

        
    }
}