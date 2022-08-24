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
                        } else { // the word is red
                            tmp_num_r[index as usize] += 1;
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
            if overall_variables::IF_ACCEPTABLE_SET.is_some() == true { // outside words
                for word in overall_variables::ACCEPTABLE_SET.iter() {

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
                            }

                        }
                    }

                    // print the words
                    if b == true {
                        print!("{} ", w);
                        print_num += 1;
                    }
                }

                println!("");
                println!("There are {} possibilities for the answer", 
                    console::style(print_num).blink().blue());
            } else {
                for i in 0 ..= builtin_words::ACCEPTABLE.len() - 1 {

                    let mut b: bool = true;
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
                            }

                        }
                    }

                    // print the words
                    if b == true {
                        print!("{} ", w);
                        print_num += 1;
                    }
                }

                println!("");
                println!("There are {} possibilities for the answer", 
                    console::style(print_num).blink().blue());
            }
        }

    }

}