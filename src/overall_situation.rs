pub mod overall_variables {

    use crate::{builtin_words, parse_json};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use crate::my_tool::tool::DifficultRecord;

    pub static mut SUCCESS_NUM: i32 = 0; // the win times
    pub static mut FAIL_NUM: i32 = 0; // the fail times
    pub static mut TOTAL_NUM: i32 = 0;
    pub static mut WORD_HISTORY: Vec<usize> = Vec::new();
    pub static mut TRY_TIMES: Vec<i32> = Vec::new();
    pub static mut USE_TIMES: Vec<(usize, i32)> = Vec::new(); 
    pub static mut IS_DAY: Option<i32> = None;
    pub static mut IS_SEED: Option<u64> = None;
    pub static mut ROUND: i32 = 0; // the round is --seed model
    pub static mut IF_CONFLICT: bool = false;
    pub static mut IF_FINAL_SET: Option<String> = None;
    pub static mut IF_ACCEPTABLE_SET: Option<String> = None;
    pub static mut ACCEPTABLE_SET: Vec<String> = Vec::new();
    pub static mut FINAL_SET: Vec<String> = Vec::new();
    
    pub static mut NEED_PARSE: bool = true; // only the first round need to parse
    pub static mut RECORD_WORD: Option<String> = None;
    pub static mut RECORD_RANDOM: bool = false;
    pub static mut RECORD_DIF: bool = false;
    pub static mut RECORD_STATS: bool = false;
    pub static mut IF_STATE: bool = false;
    pub static mut JSON_ADDRESS: String = String::new();
    pub static mut GAMES_RECORD: Vec<parse_json::process_json::Games> = Vec::new();

    pub static mut IS_CONFIG: bool = false;
    pub static mut CONFIG_ADDRESS: String = String::new();

    pub static mut CONFIG_DEF: [bool; 9] = [false; 9];

    // the senior functions
    pub static mut NEED_TIP: bool = false;
    pub static mut TIP_RECORD: Vec<Vec<DifficultRecord>> = Vec::new();

    pub fn try_times_on_average() -> f64 {
        let mut a: f64 = 0.0;
        let mut b: f64 = 0.0;
        unsafe {
            for x in TRY_TIMES.clone() {
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
            let l = USE_TIMES.len();
            let mut if_find: Option<usize> = None;
            if l > 0 {
                for i in 0 ..= l - 1 {
                    if USE_TIMES[i].0 == index.unwrap() {
                        if_find = Some(i);
                        break;
                    }
                }
            }
            if if_find.is_none() == true {
                USE_TIMES.push( (index.unwrap(), 1) );
                let mut pos = USE_TIMES.len() - 1;

                if pos > 0 {
                    while USE_TIMES[pos].1 > USE_TIMES[pos - 1].1 {
                        let t = USE_TIMES[pos].clone();
                        USE_TIMES[pos] = USE_TIMES[pos - 1].clone();
                        USE_TIMES[pos - 1] = t.clone();
                        pos -= 1;
                        if pos == 0 { break; }
                    }
                }
                if pos > 0 {
                    while USE_TIMES[pos].1 == USE_TIMES[pos - 1].1 {
                        let index1 = USE_TIMES[pos - 1].0;
                        let index2 = USE_TIMES[pos].0;
                        let s1 = builtin_words::ACCEPTABLE[index1];
                        let s2 = builtin_words::ACCEPTABLE[index2];

                        if s1 > s2 {
                            let t = USE_TIMES[pos].clone();
                            USE_TIMES[pos] = USE_TIMES[pos - 1].clone();
                            USE_TIMES[pos - 1] = t.clone();
                            pos -= 1;
                        } else {
                            break;
                        }
                        if pos == 0 { break; }
                    }
                }

            } else {
                let mut pos: usize = if_find.unwrap();
                USE_TIMES[pos].1 += 1;
                if pos > 0 {
                    while USE_TIMES[pos].1 > USE_TIMES[pos - 1].1 {
                        let t = USE_TIMES[pos].clone();
                        USE_TIMES[pos] = USE_TIMES[pos - 1].clone();
                        USE_TIMES[pos - 1] = t.clone();
                        pos -= 1;
                        if pos == 0 { break; }
                    }
                }
                if pos > 0 {
                    while USE_TIMES[pos].1 == USE_TIMES[pos - 1].1 {
                        let index1 = USE_TIMES[pos - 1].0;
                        let index2 = USE_TIMES[pos].0;
                        let s1 = builtin_words::ACCEPTABLE[index1];
                        let s2 = builtin_words::ACCEPTABLE[index2];

                        if s1 > s2 {
                            let t = USE_TIMES[pos].clone();
                            USE_TIMES[pos] = USE_TIMES[pos - 1].clone();
                            USE_TIMES[pos - 1] = t.clone();
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
            let mut l = USE_TIMES.len();
            if l > 5 { l = 5; }
            for i in 0 ..= l - 1 {
                let word = builtin_words::ACCEPTABLE[USE_TIMES[i].0];
                let num = USE_TIMES[i].1;
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
            let a: f64 = SUCCESS_NUM as f64;
            let b: f64 = (SUCCESS_NUM + FAIL_NUM) as f64;
            a / b
        }
    }

    pub fn print_frequent_test() {
        unsafe {
            let mut l = USE_TIMES.len();
            if l > 5 { l = 5; }
            for i in 0 ..= l - 1 {
                let word = builtin_words::ACCEPTABLE[USE_TIMES[i].0].to_uppercase();
                let num = USE_TIMES[i].1;
                print!("{} {}", word, num);
                if i < l - 1 { print!(" "); }
            }
            println!("");
        }
    }

    pub fn final_len() -> usize {
        unsafe {
            if IF_FINAL_SET.is_none() == true {
                2315
            } else {
                FINAL_SET.len()
            }
        }
    }

    pub fn read_acceptable_set(address: String) {

        unsafe { ACCEPTABLE_SET.clear(); }

        let file = File::open(address).unwrap();
        let reader = BufReader::new(file);

        for (_, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            //println!("Acceptable: {}", line);
            unsafe { ACCEPTABLE_SET.push(line); }
        }

    }

    pub fn read_final_set(address: String) {

        unsafe { FINAL_SET.clear(); }

        let file = File::open(address).unwrap();
        let reader = BufReader::new(file);

        for (_, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            //println!("Final: {}", line);
            unsafe { FINAL_SET.push(line); }
        }
        unsafe { FINAL_SET.sort(); }
        
    }

    
}
