pub mod average_times {

    use crate::my_tool;
    use crate::my_tool::tool::Color;
    use crate::{builtin_words::FINAL, wordle_solver::solver::start_solver};
    
    pub static mut NOW_ANS: Vec<char> = Vec::new();
    pub static mut TIMES: Vec<i32> = Vec::new();

    pub static mut IF_FIRST: bool = false;
    pub static mut FIRST_WORD: String = String::new();

    pub fn get_color(my_guess: &String) -> Vec<my_tool::tool::Color> {

        use crate::my_tool::tool::match_words;

        let mut gus: Vec<char> = Vec::new();
        for words in my_guess.clone().chars() {
            gus.push(words);
        } // read the input words and convert into chars

        let mut colors: Vec<Color> = vec![Color::Unknown; 5];//the color of XXXXX
        let mut cnt_gus: Vec<i32> = vec![0; 26];
        // the number of letters in the guessed word

        let ans: Vec<char>;
        unsafe { ans = NOW_ANS.clone(); }
        for i in 0 ..= 4 {
            if gus[i] == ans[i] {
                cnt_gus[match_words(gus[i])] += 1;
            }
        }

        let mut count: Vec<i32> = vec![0; 26];
        for i in 0 ..= 4 {
            count[match_words(ans[i])] += 1;
        }

        for i in 0 ..= 4 {
               
            if gus[i] == ans[i] {
                colors[i] = Color::Green;
            } else {

                let letter_num = match_words(gus[i]);
                cnt_gus[letter_num] += 1;
                //count the number of used letters

                if cnt_gus[match_words(gus[i])] <= count[match_words(gus[i])]
                {
                    colors[i] = Color::Yellow;
                } else {
                    colors[i] = Color::Red;
                }
            } // give the color of XXXXX

        } //process the word

        colors.clone()

    }

    pub fn test_average_guess_times() {

        let if_first: bool;
        unsafe { if_first = IF_FIRST; }
        let mut best_words: Vec<(String, f64)> = Vec::new();

        for first in 0 ..= FINAL.len() - 1 { // traverse the first word

            unsafe {
                FIRST_WORD = FINAL[first].clone().to_string();
                //println!("break 5");
                print!("When choose {} as the first word, ", &FIRST_WORD);
                //println!("break 6");
            }

            for i in 0 ..= FINAL.len() - 1 { // traverse all the words 
                // as answer
    
                let answer = FINAL[i].clone();
                if if_first == false {
                    print!("Now calculating the {} word: {},", i + 1, &answer);
                } 
                let mut ans: Vec<char> = Vec::new();
                for chars in answer.chars() {
                    ans.push(chars);
                }
                unsafe { NOW_ANS = ans.clone(); }
                start_solver();
    
                if if_first == false {
                    unsafe {
                        println!("solver takes {} times to get the answer",
                            TIMES[TIMES.len() - 1]);
                    }
                }
            }
    
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            unsafe {
                for i in 0 ..= TIMES.len() - 1 {
                    let x: f64 = TIMES[i] as f64;
                    a += x;
                    b += 1.0;
                }
            }
            if if_first == false {
                println!("Each word takes {:.2} times to guess on average.", a / b);
                break;
            } else {
                println!("guess {:.2} times on average.", a / b);
                let v: (String, f64);
                unsafe {
                    v = (FIRST_WORD.clone(), a / b as f64);
                }
                if best_words.len() < 5 {
                    best_words.push(v);
                } else {
                    best_words.push(v);
                    let mut pos: usize = 5;
                    while best_words[pos].1 < best_words[pos - 1].1 {
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

        if if_first == true {
            println!("The best initial words are:");
            for i in 0 ..= 4 {
                println!("{} takes {:.2} times to guess on average",
                    &best_words[i].0, best_words[i].1);
            }
        }
        
    }
}