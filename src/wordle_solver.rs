pub mod solver {

    use crate::overall_situation::overall_variables::TIP_RECORD;
    use crate::my_tool::tool::{DifficultRecord, match_words, Color};
    use crate::builtin_words;

    pub fn start_solver() {

        println!("{}",
            console::style("Welcome to use Wordle Solver!")
            .blink().color256(150));
        println!("You should determine a word of 5 letters.");

        let mut round: i32 = 0;
        let mut my_guess: String = "world".to_string();

        unsafe { TIP_RECORD.clear(); }

        loop {
            round += 1;
            let mut gus_record: Vec<DifficultRecord> = Vec::new();
            println!("Round {}: My guess is {}", 
                console::style(round).blink().blue(),
                console::style(&my_guess).blink().green());
            println!("Please give me the Colors (such as RRYGG)");
            let colors: String =  text_io::read!();

            let mut c: Vec<Color> = Vec::new();
            for ch in colors.chars() {
                let color: Color = match ch {
                    'G' => Color::Green,
                    'Y' => Color::Yellow,
                    'R' => Color::Red,
                    _ => Color::Unknown
                };
                c.push(color);
            } // the color vector

            let mut if_guessed: bool = true;
            for i in 0 ..= 4 {
                if c[i] != Color::Green {
                    if_guessed = false;
                    break;
                }
            }
            if if_guessed == true {
                println!("The answer is {}!", 
                    console::style(&my_guess).blink().green());
                break;
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
                    my_guess = w.clone().to_string();
                    break;
                }
            }
        }
    }
}