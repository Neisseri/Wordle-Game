use text_io::read;
use crate::my_tool::tool;

pub mod run_test_model {

    use super::tool::{match_words, Color, valid, args_parse};
    use crate::{builtin_words, my_tool::tool::match_number, overall_situation::overall_variables::{is_day, is_seed, round, if_conflict, need_parse, record_random, record_word, record_dif, record_stats, final_set, ConfigAddress}, config};
    use rand::{Rng, SeedableRng};
    use rand::rngs::StdRng;
    use rand::seq::SliceRandom;
    use crate::overall_situation::overall_variables;
    use crate::parse_json::process_json;

    pub fn test_run() -> (bool, bool, bool) {

        use crate::my_tool::tool::{ difficult_record, difficult_valid };
        use crate::config::Configuration;

        let mut is_word: Option<String> = None;
        let mut is_random: bool = false;
        let mut is_difficult: bool = false;
        let mut is_stats: bool = false;
        let mut all_guess: Vec<String> = Vec::new();

        unsafe {
            if need_parse == true { // the first circle, so we need to parse arguments
                (is_word, is_random, is_difficult, is_stats) = args_parse();
                
                if overall_variables::IsConfig == true {
                    let mut tmp_word: Option<String> = None;
                    let mut tmp_random: bool = false;
                    let mut tmp_difficult: bool = false;
                    let mut tmp_stats: bool = false;
                    (tmp_word, tmp_random, tmp_difficult, tmp_stats) = 
                        config::Configuration::parse_config(
                            ConfigAddress.clone());
                    if is_word.is_none() == true && tmp_word.is_some() == true {
                        is_word = tmp_word;
                    }
                    if tmp_random == true { is_random = true; }
                    if tmp_difficult == true { is_difficult = true; }
                    if tmp_stats == true { is_stats = true; }
                }

                record_word = is_word.clone();
                record_random = is_random;
                record_dif = is_difficult;
                record_stats = is_stats;
                need_parse = false;
            } else { // we have parsed the arguments
                is_word = record_word.clone();
                is_random = record_random;
                is_difficult = record_dif;
                is_stats = record_stats;
            }
        }

        unsafe {
            if if_conflict == true {
                panic!("Conflicted Arguments!");
            }
        } // -d -s

        unsafe {

            /*if overall_variables::if_acceptable_set.is_some() == true {
                let s: String = (overall_variables::if_acceptable_set.clone()).unwrap();
                overall_variables::read_acceptable_set(s.clone());
            }

            if overall_variables::if_final_set.is_some() == true {
                let s: String = (overall_variables::if_final_set.clone()).unwrap();
                overall_variables::read_final_set(s.clone());
            }*/

            if overall_variables::if_state == true {
                process_json::test_load_json(overall_variables::json_address.clone());
            }
        }

        let mut guess_right: bool = true;
        let mut answer: String = String::new();
        let mut dif_rec: Vec<difficult_record> = Vec::new();

        if is_random == true { // the random model: read the answer from FINAL

            let mut seed: Option<u64>;
            unsafe { seed = is_seed; } // --seed

            // --seed
            if seed.is_none() == true {

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
                answer = builtin_words::FINAL[index].to_string();

                // --day
                unsafe {
                    if is_day.is_some() == true {
                        let day_num: i32 = is_day.unwrap() - 1;
                        if day_num > 0 {
                            for _ in 1 ..= day_num {
                                let mut rng = rand::thread_rng();
                                let mut index: usize = rng.gen_range(0 ..= 2314);
                                let mut if_repeat = overall_variables::word_history.iter().position(|r| r == &index);
                                while if_repeat.is_some() == true {
                                    index = rng.gen_range(0 ..= 2314);
                                    if_repeat = overall_variables::word_history.iter().position(|r| r == &index);
                                }
                                overall_variables::word_history.push(index);
                                answer = builtin_words::FINAL[index].to_string();
                            }
                        }
                        is_day = None;
                    }
                } // --day

            } else { // have seed
                
                unsafe {
                    if overall_variables::if_final_set == None {
                        let mut rand_seed: u64 = seed.unwrap();
                        let mut rng = rand::rngs::StdRng::seed_from_u64(rand_seed);
                        let mut y = [0_usize; 2315];
                        for i in 0_usize ..= 2314_usize {
                            y[i] = i;
                        }
                        y.shuffle(& mut rng);
                        let mut index = 0;
                        round += 1;
                        if is_day.is_none() == true {
                            index = round - 1;
                        } else {
                            let mut skip = is_day.unwrap();
                            index = round - 1 + skip - 1;
                        }
                        answer = builtin_words::FINAL[y[index as usize]].to_string();
                    } else { // FINAL is read from file
                        let l = overall_variables::final_len();
                        //if l == 0 { println!("!!!!!!!!!!!!"); }
                        let mut y: Vec<usize> = Vec::new();
                        let mut rand_seed: u64 = seed.unwrap();
                        for i in 0 ..= l - 1 {
                            y.push(i);
                        }
                        y.shuffle(&mut rand::rngs::StdRng::seed_from_u64(rand_seed));
                        let mut index = 0;
                        round += 1;
                        if is_day.is_none() == true {
                            index = round - 1;
                        } else {
                            let mut skip = is_day.unwrap();
                            index = round - 1 + skip - 1;
                        }
                        answer = builtin_words::FINAL[y[index as usize]].to_string();
                        //println!("The answer is {}", answer);
                        //println!("The index is {}", y[index as usize]);
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

            if is_difficult == true && number > 1 {

                while difficult_valid(&gus, &dif_rec) == false {

                    println!("INVALID");
                    guess.clear();
                    std::io::stdin().read_line(&mut guess);
                    guess.pop();

                    while valid(&guess) == false {
                        println!("INVALID");
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

            overall_variables::record_use_times(guess.clone());

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
            //println!("case 2");

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
                    overall_variables::try_times.push(number as i32);
                }
                break;
            }
            //println!("case 3");
        }

        if guess_right == false {
            unsafe {
                overall_variables::fail_num += 1;
                overall_variables::TotalNum += 1;
            }
            println!("FAILED {}", answer.to_uppercase());
        } else {
            unsafe {
                overall_variables::success_num += 1;
                overall_variables::TotalNum += 1; 
            }
        }

        unsafe {
            overall_variables::GamesRecord.push(
                crate::parse_json::process_json::Games {
                    answer: answer.clone().to_uppercase(),
                    guesses: all_guess.clone()
                }
            );
        }

        let mut word: bool = false;
        if is_word.is_some() == true {
            word = true;
        }
        //println!("case 4");
        (word, is_random, is_stats)

    }
}