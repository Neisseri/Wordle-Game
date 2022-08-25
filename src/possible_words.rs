pub mod pos_word {

    use crate::builtin_words;
    use crate::overall_situation::overall_variables;
    use crate::{overall_situation::overall_variables::TIP_RECORD};
    use crate::my_tool::tool::{Color, match_words};

    pub fn print_ps_words() {

        let mut limit: Vec<i32> = vec![-1; 5];
        let mut num: Vec<i32> = vec![0; 26]; // the number of 26 words
        let mut if_exact: Vec<bool> = vec![false; 26];
        let mut print_num = 0;
        // if the number of word == num[i]
        let mut not_this_place :Vec<Vec<bool>> = Vec::new();
        for _ in 0 ..= 4 {
            let v: Vec<bool> = vec![false; 26];
            not_this_place.push(v);
        } // record on place i the word j is not correct

        let mut info_cmp: Vec<i32> = vec![1; 26];
        // compare how many information we know about the word
        // the less the better
        let mut best_words: Vec<(i32, &str)> = Vec::new();
        // Vec< their info, their code>

        unsafe {
            for i in 0 ..= TIP_RECORD.len() - 1 {

                let mut tmp_num_y: Vec<i32> = vec![0; 26];
                let mut tmp_num_r: Vec<i32> = vec![0; 26];
                let mut tmp_num_g: Vec<i32> = vec![0; 26];

                for j in 0 ..= 4 {
                    let index = TIP_RECORD[i][j].letter;

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
        unsafe {
            if overall_variables::IF_ACCEPTABLE_SET.is_some() == true { 
                // get the accetpable words from outside files
                
                let mut best_words_string: Vec<(i32, String)> = Vec::new();
                // Vec< their info, their code>
                
                for word in overall_variables::ACCEPTABLE_SET.iter() {

                    let mut info: i32 = 0;

                    let mut b: bool = true;
                    let w: String = word.clone();
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
                        print!("{} ", w);
                        print_num += 1;
                        for j in 0 ..= 4 {
                            info += info_cmp[match_words(ch[j])];
                        }
                        if best_words.len() < 5 {
                            best_words_string.push((info, w.clone()));
                        } else {
                            let v = (info, w.clone());
                            best_words_string.push(v);
                            let mut pos: usize = 5;
                            while best_words[pos].0 < best_words[pos - 1].0 {
                                let t = best_words[pos].clone();
                                best_words[pos] = best_words[pos - 1].clone();
                                best_words[pos - 1] = t.clone();
                                pos -= 1;
                                if pos == 0 { break; }
                            }
                            best_words.pop();
                        }
                    }
                }

                println!("");
                println!("There are {} possibilities for the answer", 
                    console::style(print_num).blink().blue());

                    if overall_variables::NEED_RECOMMEND == true {
                        println!("{}",
                            console::style("The recommended words are:").blink().yellow());
                        for i in 0 ..= best_words.len() - 1 {
                            print!("{} ", 
                                console::style(&best_words[i].1).blink().green());
                        }
                        println!("");
                    }

            } else {
                // read words from the built-in ACCEPTABLE
                for i in 0 ..= builtin_words::ACCEPTABLE.len() - 1 {

                    let mut b: bool = true;
                    let mut info: i32 = 0;
                    let w = builtin_words::ACCEPTABLE[i].clone();
                    let mut ch: Vec<char> = Vec::new();
                    for chars in w.chars() {
                        ch.push(chars);
                    }
                    for j in 0 ..= 4 {
                        if limit[j] >= 0 {
                            if limit[j] != match_words(ch[j]) as i32 {
                                b = false;
                                break;
                            }
                        }
                        if not_this_place[j][match_words(ch[j])] == true {
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
                        print!("{} ", w);
                        print_num += 1;
                        for j in 0 ..= 4 {
                            //println!("{}", ch[j]);
                            info += info_cmp[match_words(ch[j])];
                        }
                        if best_words.len() < 5 {
                            best_words.push((info, w.clone()));
                        } else {
                            let v = (info, w.clone());
                            best_words.push(v);
                            let mut pos: usize = 5;
                            while best_words[pos].0 < best_words[pos - 1].0 {
                                let t = best_words[pos].clone();
                                best_words[pos] = best_words[pos - 1].clone();
                                best_words[pos - 1] = t.clone();
                                pos -= 1;
                                if pos == 0 { break; }
                            }
                            best_words.pop();
                        }
                    }
                }

                println!("");
                println!("There are {} possibilities for the answer", 
                    console::style(print_num).blink().blue());

                if overall_variables::NEED_RECOMMEND == true {
                    println!("{}",
                        console::style("The recommended words are:").blink().yellow());
                    for i in 0 ..= best_words.len() - 1 {
                        print!("{} ", 
                            console::style(&best_words[i].1).blink().green());
                    }
                    println!("");
                }
                
            }
        }

    }

}