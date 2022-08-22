pub mod overall_variables {

    use crate::builtin_words;

    pub static mut success_num: i32 = 0;
    pub static mut fail_num: i32 = 0;
    pub static mut word_history: Vec<usize> = Vec::new();
    pub static mut try_times: Vec<i32> = Vec::new();
    pub static mut use_times: Vec<(usize, i32)> = Vec::new(); 

    pub fn try_times_on_average() -> f64 {
        let mut a: f64 = 0.0;
        let mut b: f64 = 0.0;
        unsafe {
            for x in try_times.clone() {
                a += x as f64;
                b += 1.0;
            }
        }
        if b == 0.0 {
            0.0
        } else {
            a / b
        }
    }

    pub fn record_use_times(word: String) {

        let index = builtin_words::ACCEPTABLE.iter().
            position(|r| r == &(word.as_str()));
        // the index of the word
        // then we need to find if it's in `use_times`
        
        unsafe { 
            let l = use_times.len();
            let mut if_find: Option<usize> = None;
            if l > 0 {
                for i in 0 ..= l - 1 {
                    if use_times[i].0 == index.unwrap() {
                        if_find = Some(i);
                        break;
                    }
                }
            }
            if if_find.is_none() == true {
                use_times.push( (index.unwrap(), 1) );
                let mut pos = use_times.len() - 1;

                if pos > 0 {
                    while use_times[pos].1 > use_times[pos - 1].1 {
                        let mut t = use_times[pos].clone();
                        use_times[pos] = use_times[pos - 1].clone();
                        use_times[pos - 1] = t.clone();
                        pos -= 1;
                        if pos == 0 { break; }
                    }
                }
                if pos > 0 {
                    while use_times[pos].1 == use_times[pos - 1].1 {
                        let mut index1 = use_times[pos - 1].0;
                        let mut index2 = use_times[pos].0;
                        let mut s1 = builtin_words::ACCEPTABLE[index1];
                        let mut s2 = builtin_words::ACCEPTABLE[index2];

                        if s1 > s2 {
                            let mut t = use_times[pos].clone();
                            use_times[pos] = use_times[pos - 1].clone();
                            use_times[pos - 1] = t.clone();
                            pos -= 1;
                        } else {
                            break;
                        }
                        if pos == 0 { break; }
                    }
                }

            } else {
                let mut pos: usize = if_find.unwrap();
                use_times[pos].1 += 1;
                if pos > 0 {
                    while use_times[pos].1 > use_times[pos - 1].1 {
                        let mut t = use_times[pos].clone();
                        use_times[pos] = use_times[pos - 1].clone();
                        use_times[pos - 1] = t.clone();
                        pos -= 1;
                        if pos == 0 { break; }
                    }
                }
                if pos > 0 {
                    while use_times[pos].1 == use_times[pos - 1].1 {
                        let mut index1 = use_times[pos - 1].0;
                        let mut index2 = use_times[pos].0;
                        let mut s1 = builtin_words::ACCEPTABLE[index1];
                        let mut s2 = builtin_words::ACCEPTABLE[index2];

                        if s1 > s2 {
                            let mut t = use_times[pos].clone();
                            use_times[pos] = use_times[pos - 1].clone();
                            use_times[pos - 1] = t.clone();
                            pos -= 1;
                        } else { break; }
                        if pos == 0 { break; }
                    }
                }
            }
        }

    }

    pub fn print_frequent() {
        unsafe {
            let mut l = use_times.len();
            if l > 5 { l = 5; }
            for i in 0 ..= l - 1 {
                let word = builtin_words::ACCEPTABLE[use_times[i].0];
                let num = use_times[i].1;
                print!("{} {}",
                    console::style(word.to_uppercase()).bold().blink().green(),
                    console::style(num).bold().blink().blue()
                );
                if i < l - 1 { print!(" "); }
            }
            println!("");
        }
    }

    pub fn win_chance() -> f64 {
        unsafe {
            let a: f64 = success_num as f64;
            let b: f64 = (success_num + fail_num) as f64;
            a / b
        }
    }

    pub fn print_frequent_test() {
        unsafe {
            let mut l = use_times.len();
            if l > 5 { l = 5; }
            for i in 0 ..= l - 1 {
                let word = builtin_words::ACCEPTABLE[use_times[i].0].to_uppercase();
                let num = use_times[i].1;
                print!("{} {}", word, num);
                if i < l - 1 { print!(" "); }
            }
            println!("");
        }
    }
}