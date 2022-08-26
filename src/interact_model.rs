pub mod run_interact_model {

    use crate::{my_tool::{tool::{self, DifficultRecord}}, builtin_words::FINAL};

    pub fn interact_run() -> (bool, bool, bool) {

        use crate::my_tool::tool::{ match_words, Color, valid, args_parse };
        use rand::{Rng, SeedableRng};
        use rand::seq::SliceRandom;
        use crate::builtin_words;
        use crate::overall_situation::overall_variables;
        use crate::{my_tool::tool::match_number,
            overall_situation::overall_variables::{
                IS_DAY, IS_SEED, ROUND, IF_CONFLICT,
                NEED_PARSE, RECORD_RANDOM, RECORD_WORD,
                RECORD_DIF, RECORD_STATS, CONFIG_ADDRESS
            }, config};
        use crate::parse_json::process_json;
        use crate::possible_words::pos_word;

        let mut is_word: Option<String>;
        let mut is_random: bool;
        let mut is_difficult: bool;
        let mut is_stats: bool;
        let mut all_guess_str: Vec<String> = Vec::new();
        
        unsafe {

            overall_variables::TIP_RECORD.clear();

            if NEED_PARSE == true { // the first circle, so we need to parse arguments
                (is_word, is_random, is_difficult, is_stats) = args_parse();
                
                if overall_variables::IS_CONFIG == true {
                    let tmp_word: Option<String>;
                    let tmp_random: bool;
                    let tmp_difficult: bool;
                    let tmp_stats: bool;
                    (tmp_word, tmp_random, tmp_difficult, tmp_stats) = 
                        config::configuration::parse_config(
                            CONFIG_ADDRESS.clone());
                    if is_word.is_none() == true && tmp_word.is_some() == true {
                        is_word = tmp_word;
                    }
                    if tmp_random == true { is_random = true; }
                    if tmp_difficult == true { is_difficult = true; }
                    if tmp_stats == true { is_stats = true; }
                }

                RECORD_WORD = is_word.clone();
                RECORD_RANDOM = is_random;
                RECORD_DIF = is_difficult;
                RECORD_STATS = is_stats;
                NEED_PARSE = false;

            } else { // we have parsed the arguments
                is_word = RECORD_WORD.clone();
                is_random = RECORD_RANDOM;
                is_difficult = RECORD_DIF;
                is_stats = RECORD_STATS;
            }
        }

        if is_random == true {
            println!("This is the {}!",
                console::style("Random Model").blink().green());
        }

        if is_difficult == true {
            println!("This is the {}!",
                console::style("Difficult Model").blink().green());
        }

        unsafe {
            if IF_CONFLICT == true {
                println!("{}", console::style("You have type in conflicted arguments!")
                    .blink().red());
                panic!("Conflicted Arguments!");
            } // -d -s

            if overall_variables::IF_STATE == true {
                process_json::test_load_json(overall_variables::JSON_ADDRESS.clone());
            } // if the json is illegal, we need to panic
        }

        let mut guess_right: bool = true;
        let mut answer: String = String::new();

        if is_random == true { // the random model: read the answer from FINAL

            let seed: Option<u64>;
            unsafe { seed = IS_SEED; }
            // extract the seed from the static variable 
            // to reduce the use of `unsafe`

            if seed.is_none() == true { // no --seed argument
            
                let rng = &mut rand::rngs::StdRng::seed_from_u64(10);
                let mut index: usize = rng.gen_range(0 ..= 2314);
                unsafe {
                    let mut if_repeat = overall_variables::WORD_HISTORY.iter().position(|r| r == &index);
                    while if_repeat.is_some() == true {
                        index = rng.gen_range(0 ..= 2314);
                        if_repeat = overall_variables::WORD_HISTORY.iter().position(|r| r == &index);
                    }
                    overall_variables::WORD_HISTORY.push(index);
                }
                answer = FINAL[index].to_string();

                // --day
                unsafe {
                    if IS_DAY.is_some() == true { // need to skip some rounds
                        let day_num: i32 = IS_DAY.unwrap() - 1;
                        if day_num > 0 {
                            for _ in 1 ..= day_num {
                                let mut rng = rand::thread_rng();
                                let mut index: usize = rng.gen_range(0 ..= 2314);
                                let mut if_repeat = overall_variables::WORD_HISTORY.iter().position(|r| r == &index);
                                while if_repeat.is_some() == true {
                                    index = rng.gen_range(0 ..= 2314);
                                    if_repeat = overall_variables::WORD_HISTORY.iter().position(|r| r == &index);
                                }
                                overall_variables::WORD_HISTORY.push(index);
                                answer = builtin_words::FINAL[index].to_string();
                            }
                        }
                        IS_DAY = None;
                    }
                } // --day

            } else { // use seed to generate the answers
                unsafe {
                    if overall_variables::IF_FINAL_SET == None {
                        let rand_seed: u64 = seed.unwrap();
                        let mut rng = rand::rngs::StdRng::seed_from_u64(rand_seed);
                        let mut y = [0_usize; 2315];
                        for i in 0_usize ..= 2314_usize {
                            y[i] = i;
                        }
                        y.shuffle(& mut rng);
                        let index;
                        ROUND += 1;
                        if IS_DAY.is_none() == true {
                            index = ROUND - 1;
                        } else {
                            let skip = IS_DAY.unwrap();
                            index = ROUND - 1 + skip - 1;
                        }
                        answer = builtin_words::FINAL[y[index as usize]].to_string();
                    } else { // FINAL is read from file
                        let l = overall_variables::final_len();
                        //if l == 0 { println!("!!!!!!!!!!!!"); }
                        let mut y: Vec<usize> = Vec::new();
                        let rand_seed: u64 = seed.unwrap();
                        for i in 0 ..= l - 1 {
                            y.push(i);
                        }
                        y.shuffle(&mut rand::rngs::StdRng::seed_from_u64(rand_seed));
                        let index;
                        ROUND += 1;
                        if IS_DAY.is_none() == true {
                            index = ROUND - 1;
                        } else {
                            let skip = IS_DAY.unwrap();
                            index = ROUND - 1 + skip - 1;
                        }
                        answer = builtin_words::FINAL[y[index as usize]].to_string();
                    }
                }
            }
        } else if is_word.is_some() == true { // read the answer from args
            
            answer = is_word.clone().unwrap().to_lowercase();

        } else { // the basic model

            // read in answer from stdin
            println!("Please type in the Answer!");
            
            std::io::stdin().read_line(&mut answer).expect("INPUT ERROR");
            answer.pop();
            while valid(&answer) == false {
                println!("{}", console::style("The word is invalid!").bold().red());
                answer.clear();
                std::io::stdin().read_line(&mut answer).expect("INPUT ERROR");
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

        let mut dif_rec: Vec<DifficultRecord> = Vec::new();

        // the clone Triat is not convenient, so use another vector
        // to cover the data for `TIP_RECORD`
        
        for number in 1 ..= 6 {

            let mut dif_rec2: Vec<DifficultRecord> = Vec::new();

            println!("");
            println!("This is round {}:", number);

            unsafe {
                if overall_variables::NEED_TIP == true {
                    if number == 1 {
                        println!("{}", 
                            console::style("All words are possible!")
                            .blink().yellow());
                    } else {
                        println!("{}", 
                            console::style("All the possible words are:")
                            .blink().yellow());
                        pos_word::print_ps_words();
                    }
                } else if overall_variables::NEED_RECOMMEND == true {
                    if number > 1 {
                        pos_word::print_ps_words();
                    }
                }
            }

            guess_right = true;

            guess.clear();
            println!("{}", 
                console::style("Please guess the word!")
                .blink().color256(50));
            std::io::stdin().read_line(&mut guess).expect("INPUT ERROR");
            guess.pop();
            
            while valid(&guess) == false {
                println!("{}", console::style("The word is invalid!").bold().red());
                guess.clear();
                std::io::stdin().read_line(&mut guess).expect("INPUT ERROR");
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
                    std::io::stdin().read_line(&mut guess).expect("INPUT ERROR");
                    guess.pop();

                    while valid(&guess) == false {
                        println!("{}", console::style("The word is invalid!").bold().red());
                        guess.clear();
                        std::io::stdin().read_line(&mut guess).expect("INPUT ERROR");
                        guess.pop();
                    }

                    gus.clear();
                    for words in guess.chars() {
                        gus.push(words);
                    } // read the input words and convert into chars
                }

            }

            overall_variables::record_use_times(guess.clone());
            // used to print the state of game-history

            all_guess_str.push(guess.clone().to_uppercase());
            // the all_guess_str is for the json
            // so it need String type

            all_guess.push(gus.clone());

            let mut colors: Vec<Color> = vec![Color::Unknown; 5];//the color of XXXXX

            let mut cnt_gus: Vec<i32> = vec![0; 26];
            // the number of letters in the guessed word

            dif_rec.clear();
            dif_rec2.clear();

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

                dif_rec.push(DifficultRecord{
                    letter: match_words(gus[i]) as i32,
                    color: colors[i].clone()
                });
                dif_rec2.push(DifficultRecord{
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

            unsafe {
                overall_variables::TIP_RECORD.push(dif_rec2);
            }

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
                    overall_variables::TRY_TIMES.push(number as i32);
                }
                break;
            }
        }

        if guess_right == false {
            unsafe {
                overall_variables::FAIL_NUM += 1;
                overall_variables::TOTAL_NUM += 1; 
            }
            println!("You failed! The answer is {}!", answer.to_uppercase());
        } else {
            unsafe {
                overall_variables::SUCCESS_NUM += 1;
                overall_variables::TOTAL_NUM += 1;  
            }
        }

        unsafe {
            overall_variables::GAMES_RECORD.push(
                crate::parse_json::process_json::Games {
                    answer: answer.clone().to_uppercase(),
                    guesses: all_guess_str.clone()
                }
            );
        } // the GamesRecord will be weitten into the json

        let mut word: bool = false;
        if is_word.is_some() == true {
            word = true;
        }
        (word, is_random, is_stats)

        
    }
}