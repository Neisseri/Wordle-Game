pub mod solver {

    use crate::overall_situation::overall_variables::{TIP_RECORD, self, IF_CALCULATE};
    use crate::my_tool::tool::{DifficultRecord, match_words, Color, modify};
    use crate::{builtin_words, test_solver};

    pub fn start_solver() {

        let if_c: bool;
        unsafe { if_c = overall_variables::IF_CALCULATE; }

        if if_c == false {
            println!("{}",
                console::style("Welcome to use Wordle Solver!")
                .blink().color256(150));
            println!("You should determine a word of 5 letters.");
        }

        let mut info_cmp: Vec<i32> = vec![1; 26];
        // compare how many information we know about the word
        // the less the better
        

        let mut round: i32 = 0;
        let mut ro: i32 = 0;
        let mut my_guess: String;
        unsafe {
            if IF_CALCULATE == true {
                my_guess = test_solver::average_times::FIRST_WORD.clone();
            } else {
                my_guess = "world".to_string();
            }
        }
        let mut not_this_place :Vec<Vec<bool>> = Vec::new();
        for _ in 0 ..= 4 {
            let v: Vec<bool> = vec![false; 26];
            not_this_place.push(v);
        } // record on place i the word j is not correct

        // record the appear times to get the recommended words

        unsafe { TIP_RECORD.clear(); }

        loop {
            round += 1;
            let mut gus_record: Vec<DifficultRecord> = Vec::new();
            let colors: String;
            let mut c: Vec<Color> = Vec::new();
            ro = modify(ro);
            if if_c == false {
                println!("Round {}: My guess is {}", 
                    console::style(round).blink().blue(),
                    console::style(&my_guess).blink().green());
                println!("Please give me the Colors (such as RRYGG)");
                colors =  text_io::read!();
                for ch in colors.chars() {
                    let color: Color = match ch {
                        'G' => Color::Green,
                        'Y' => Color::Yellow,
                        'R' => Color::Red,
                        _ => Color::Unknown
                    };
                    c.push(color);
                } // the color vector
            } else {
                c = crate::test_solver::average_times::get_color(&my_guess);
            }

            let mut if_guessed: bool = true;
            for i in 0 ..= 4 {
                if c[i] != Color::Green {
                    if_guessed = false;
                    break;
                }
            }
            if if_c == false {
                if if_guessed == true {
                    println!("The answer is {}!", 
                        console::style(&my_guess).blink().green());
                    break;
                }
            } else {
                if if_guessed == true {
                    unsafe { test_solver::average_times::TIMES.push(ro); }
                    break;
                }
            }
            

            let mut letter: Vec<i32> = Vec::new();
            for l in my_guess.chars() {
                letter.push(match_words(l) as i32);
            }

            for i in 0 ..= 4 {
                gus_record.push(DifficultRecord{
                    letter: letter[i],
                    color: c[i].clone()
                });
            }

            unsafe { TIP_RECORD.push(gus_record); }

            // then look for a possible answer

            let mut limit: Vec<i32> = vec![-1; 5];
            let mut num: Vec<i32> = vec![0; 26]; // the number of 26 words
            let mut if_exact: Vec<bool> = vec![false; 26];
            // if the number of word == num[i]

            unsafe {
                for i in 0 ..= TIP_RECORD.len() - 1 {

                    let mut tmp_num_y: Vec<i32> = vec![0; 26];
                    let mut tmp_num_r: Vec<i32> = vec![0; 26];
                    let mut tmp_num_g: Vec<i32> = vec![0; 26];

                    for j in 0 ..= 4 {
                        let index = TIP_RECORD[i][j].letter;
                        // the code of the letter on position j

                        if TIP_RECORD[i][j].color == Color::Green {
                                limit[j] = TIP_RECORD[i][j].letter;
                                tmp_num_g[index as usize] += 1;
                        } else if TIP_RECORD[i][j].color == Color::Yellow {
                                tmp_num_y[index as usize] += 1;
                                not_this_place[j][index as usize] = true;

                                info_cmp[index as usize] += 2;
                            } else { // the word is red
                                tmp_num_r[index as usize] += 1;

                                info_cmp[index as usize] += 5;
                            }

                    }

                    for j in 0 ..= 25 {
                        if if_exact[j] == false {

                            if tmp_num_r[j] == 0 { // no red
                                if tmp_num_y[j] > num[j] {
                                    num[j] = tmp_num_y[j] + tmp_num_g[j];
                                }
                            } else { // have red
                                if_exact[j] = true;
                                num[j] = tmp_num_y[j] + tmp_num_g[j];
                            }
                        }
                    }

                }
            }

            // next we check all the acceptable words
            let mut least_info: i32 = -1;
            for i in 0 ..= builtin_words::ACCEPTABLE.len() - 1 {

                let mut b: bool = true;
                let mut info: i32 = 0;
                let w = builtin_words::ACCEPTABLE[i].clone();
                let mut ch: Vec<char> = Vec::new();
                for chars in w.chars() {
                    ch.push(chars);
                }
                for i in 0 ..= 4 {
                    if limit[i] >= 0 {
                        if limit[i] != match_words(ch[i]) as i32 {
                            b = false;
                            break;
                        }
                    }
                    if not_this_place[i][match_words(ch[i])] == true {
                        b = false;
                        break;
                    }
                }

                if b == true {
                    let mut rec_num: Vec<i32> = vec![0; 26];
                    for i in 0 ..= 4 {
                        rec_num[match_words(ch[i])] += 1;
                    }

                    for i in 0 ..= 25 { // check all the words
                        if if_exact[i] == true {
                            if num[i] != rec_num[i] {
                                b = false;
                                break;
                            }
                        } else {
                            if num[i] > rec_num[i] {
                                b = false;
                                break;
                            }
                            if num[i] < rec_num[i] {
                                info += 3;
                            }
                        }

                    }
                }

                // print the words
                if b == true {
                    for j in 0 ..= 4 {
                        //println!("{}", ch[j]);
                        info += info_cmp[match_words(ch[j])];
                    }
                    if least_info == -1 {
                        my_guess = w.clone().to_string();
                        least_info = info;
                    } else {
                        if info < least_info {
                            my_guess =w.clone().to_string();
                        }
                    }
                    break; // delete the break or not
                }
            }
        }
    }
}