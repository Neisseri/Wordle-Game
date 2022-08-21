use text_io::read;

pub mod run_interact_model {
    use crate::test_model::tool::{self, match_number};


    pub fn interact_run() -> () {

        use crate::test_model::tool::{ match_words, Color, valid };
        use console;

        let mut guess_right: bool = true;
        println!("Please type in the Answer!");
        let mut answer: String = String::new();

        std::io::stdin().read_line(&mut answer);
        answer.pop();
        while valid(&answer) == false {
            println!("{}", console::style("The word is invalid!").bold().red());
            answer.clear();
            std::io::stdin().read_line(&mut answer);
            answer.pop();
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
        
        for number in 1 ..= 6 {

            println!("This is round {}:", number);

            guess_right = true;

            guess.clear();
            std::io::stdin().read_line(&mut guess);
            guess.pop();
            //println!("{}", guess);
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

            all_guess.push(gus.clone());

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
                    Color::Unknown => print!("X")
                }

            }
            println!(""); // print the keyboard

            }

            if guess_right == true {
                println!("You used {} chances and get the answer!", number);
                break;
            }
        }

        if guess_right == false {
            println!("You failed! The answer is {}!", answer.to_uppercase());
        }
    }
}