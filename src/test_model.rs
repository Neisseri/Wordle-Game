use crate::my_tool::tool;

pub mod run_test_model {

    use super::tool::{match_words, Color, valid, args_parse};
    use crate::{builtin_words, 
        overall_situation::overall_variables::{
            IS_DAY, IS_SEED, ROUND, IF_CONFLICT, 
            NEED_PARSE, RECORD_RANDOM, RECORD_WORD, 
            RECORD_DIF, RECORD_STATS, CONFIG_ADDRESS
        }, config};
    use rand::{Rng, SeedableRng};
    use rand::seq::SliceRandom;
    use crate::overall_situation::overall_variables;
    use crate::parse_json::process_json;

    pub fn test_run() -> (bool, bool, bool) {

        use crate::my_tool::tool::{ DifficultRecord, difficult_valid };

        let mut is_word: Option<String>;
        let mut is_random: bool;
        let mut is_difficult: bool;
        let mut is_stats: bool;
        let mut all_guess: Vec<String> = Vec::new();

        unsafe {
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

        unsafe {
            if IF_CONFLICT == true {
                panic!("Conflicted Arguments!");
            } // -d -s
   
            if overall_variables::IF_STATE == true {
                process_json::test_load_json(
                    overall_variables::JSON_ADDRESS.clone());
            } // if the json is illegal, we need to panic
        }

        let mut guess_right: bool = true;
        let mut answer: String;
        let mut dif_rec: Vec<DifficultRecord> = Vec::new();

        if is_random == true { // the random model: read the answer from FINAL

            let seed: Option<u64>;
            unsafe { seed = IS_SEED; }
            // extract the seed from the static variable 
            // to reduce the use of `unsafe`

            if seed.is_none() == true { // no --seed argument

                let mut rng = rand::thread_rng();
                let mut index: usize = rng.gen_range(0 ..= 2314);
                unsafe {
                    let mut if_repeat = overall_variables::WORD_HISTORY.iter().position(|r| r == &index);
                    while if_repeat.is_some() == true {
                        index = rng.gen_range(0 ..= 2314);
                        if_repeat = overall_variables::WORD_HISTORY.iter().position(|r| r == &index);
                    }
                    overall_variables::WORD_HISTORY.push(index);
                }
                answer = builtin_words::FINAL[index].to_string();

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
            } // --seed

        } else if is_word.is_some() == true { // read the answer from args
            
            answer = is_word.clone().unwrap().to_lowercase();

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

            //println!("{}", number);
            guess_right = true;

            guess.clear();
            std::io::stdin().read_line(&mut guess).expect("INPUT ERROR");
            guess.pop();
            
            while valid(&guess) == false {
                println!("INVALID");
                guess.clear();
                std::io::stdin().read_line(&mut guess).expect("INPUT ERROR");
                guess.pop();
            }

            let mut gus: Vec<char> = Vec::new();
            for words in guess.chars() {
                gus.push(words);
            } // read the input words and convert into chars

            if is_difficult == true && number > 1 {

                while difficult_valid(&gus, &dif_rec) == false {

                    println!("INVALID");
                    guess.clear();
                    std::io::stdin().read_line(&mut guess).expect("INPUT ERROR");
                    guess.pop();

                    while valid(&guess) == false {
                        println!("INVALID");
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

            all_guess.push(guess.clone().to_uppercase());

            let mut colors: Vec<Color> = vec![Color::Unknown; 5];//the color of XXXXX

            let mut cnt_gus: Vec<i32> = vec![0; 26];
            // the number of letters in the guessed word

            dif_rec.clear();

            //println!("case 1");
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
            println!("FAILED {}", answer.to_uppercase());
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
                    guesses: all_guess.clone()
                }
            );
        } // the GamesRecord will be weitten into the json

        let mut word: bool = false;
        if is_word.is_some() == true {
            word = true;
        }
        //println!("case 4");
        (word, is_random, is_stats)

    }
}