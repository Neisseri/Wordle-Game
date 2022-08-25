pub mod tool {
    use crate::overall_situation::overall_variables::
    {IS_DAY, IS_SEED, IF_CONFLICT, IF_FINAL_SET,
        IF_ACCEPTABLE_SET, self, FINAL_SET, ACCEPTABLE_SET,
        IF_STATE, JSON_ADDRESS, IS_CONFIG, CONFIG_ADDRESS
    };

    pub static mut PLUS: i32 = 0;

    pub fn match_words(letter: char) -> usize {
        match letter {
            'a' => 0, 'b' => 1, 'c' => 2, 'd' => 3, 'e' => 4,
            'f' => 5, 'g' => 6, 'h' => 7, 'i' => 8, 'j' => 9,
            'k' => 10, 'l' => 11, 'm' => 12, 'n' => 13, 'o' => 14,
            'p' => 15, 'q' => 16, 'r' => 17, 's' => 18, 't' => 19,
            'u' => 20, 'v' => 21, 'w' => 22, 'x' => 23, 'y' => 24,
            'z' => 25,
            _ => panic!("No this word!")
        }
    }

    pub fn match_number(num: usize) -> char {
        match num {
            0 => 'A', 1 => 'B', 2 => 'C', 3 => 'D', 4 => 'E',
            5 => 'F', 6 => 'G', 7 => 'H', 8 => 'I', 9 => 'J',
            10 => 'K', 11 => 'L', 12 => 'M', 13 => 'N', 14 => 'O',
            15 => 'P', 16 => 'Q', 17 => 'R', 18 => 'S', 19 => 'T',
            20 => 'U', 21 => 'V', 22 => 'W', 23 => 'X', 24 => 'Y',
            25 => 'Z', _ => {
                panic!("Impossible!");
            } 
        }
    }

    #[derive(Clone)]
    #[derive(PartialEq)]
    pub enum Color {
        Red,
        Yellow,
        Green,
        Unknown
    }

    pub fn valid(guess: &String) -> bool {

        use crate::builtin_words;

        if guess.chars().count() == 5_usize {

            for words in guess.chars() {
                match words {
                    'a' ..= 'z' => (),
                    _ =>  return false
                }
            }

            let find: Option<usize>;
            unsafe {
                if IF_ACCEPTABLE_SET.is_some() == true {
                    find = overall_variables::ACCEPTABLE_SET.iter().position(|r| r == guess.as_str())
                } else {
                    find = builtin_words::ACCEPTABLE.iter()
                .position(|&r| r == guess.as_str());
                }
            }
            
            if find.is_none() == true {
                return false
            }

        } else {
            return false
        }

        true
    }

    pub struct DifficultRecord {
        pub letter: i32,
        pub color: Color
    }

    pub fn difficult_valid(
        gus: &Vec<char>,
        dif_rec: &Vec<DifficultRecord>
    ) -> bool {
        let mut is_valid: bool = true;

        let mut is_used: Vec<bool> = vec![false; 5];

        for i in 0 ..= 4 {

            if dif_rec[i].color == Color::Green {
                if dif_rec[i].letter != match_words(gus[i]) as i32 {
                    is_valid = false;
                    break;
                }
                is_used[i] = true;
            }

            if dif_rec[i].color == Color::Yellow {
                let mut if_find: bool = false;
                for j in 0 ..= 4 {
                    if is_used[j] == false 
                    && dif_rec[i].letter == match_words(gus[j]) as i32 {
                        is_used[j] = true;
                        if_find = true;
                        break;
                    } 
                }
                if if_find == false {
                    is_valid = false;
                    break;
                }
            }
            
        }

        is_valid
    }

    pub fn args_parse() -> (Option<String>, bool, bool, bool) {
        let mut word: Option<String> = None;
        let mut random: bool = false;
        let mut difficult: bool = false;
        let mut stats: bool = false;

        let mut get_word: bool = false;
        let mut get_day: bool = false;
        let mut get_seed: bool = false;
        let mut get_final: bool = false;
        let mut get_acceptable: bool = false;
        let mut get_state: bool = false;
        let mut get_config: bool = false;

        for arg in std::env::args() {

            if get_word == true {
                word = Some(arg);
                get_word = false;
                continue;
            }

            if get_day == true {
                let days: Option<i32> = if_number(arg.clone());
                if days.is_some() == true {
                    unsafe { IS_DAY = days; }
                }
                get_day = false;
                continue;
            }

            if get_seed == true {
                let seed: Option<u64> = if_number_u64(arg.clone());
                if seed.is_some() == true {
                    unsafe { IS_SEED = seed; }
                }
                get_seed = false;
                continue;
            }

            if get_final == true {
                unsafe {
                    IF_FINAL_SET = Some(arg.clone());
                    overall_variables::read_final_set(arg.clone());
                }
                get_final = false;
                continue;
            }

            if get_acceptable == true {
                unsafe { 
                    IF_ACCEPTABLE_SET = Some(arg.clone()); 
                    overall_variables::read_acceptable_set(arg.clone());
                }
                get_acceptable = false;
                continue;
            }

            if get_state == true {
                unsafe {
                    JSON_ADDRESS = arg.clone();
                }
                get_state = false;
                continue;
            }

            if get_config == true {
                unsafe {
                    CONFIG_ADDRESS = arg.clone();
                }
                get_config = false;
                continue;
            }

            match arg.as_str() {
                "-w" | "--word" => { get_word = true;
                    unsafe { overall_variables::CONFIG_DEF[8] = true; } },
                "-r" | "--random" => { random = true;
                    unsafe { overall_variables::CONFIG_DEF[0] = true; } },
                "-D" | "--difficult" =>  { difficult = true;
                    unsafe { overall_variables::CONFIG_DEF[1] = true; } },
                "-t" | "--stats" => { stats = true;
                    unsafe { overall_variables::CONFIG_DEF[2] = true; } },
                "-d" | "--day" => unsafe { IS_DAY = Some(1);
                    get_day = true;
                    overall_variables::CONFIG_DEF[3] = true; },
                "-s" | "--seed" => unsafe { IS_SEED = Some(100);
                    get_seed = true;
                    overall_variables::CONFIG_DEF[4] = true; },
                "-f" | "--final-set" => { get_final = true;
                    unsafe { overall_variables::CONFIG_DEF[5] = true; } },
                "-a" | "--acceptable-set" => { get_acceptable = true;
                    unsafe { overall_variables::CONFIG_DEF[6] = true; } },
                "-S" | "--state" => unsafe { IF_STATE = true;
                    get_state = true;
                    overall_variables::CONFIG_DEF[7] = true; },
                "-c" | "--config" => unsafe { IS_CONFIG = true; get_config = true },
                _ => ()
            }
        }

        if random == true { // random model
            if word.is_some() == true {
                unsafe { IF_CONFLICT = true; }
            }
        } else { // set-answer model
            unsafe {
                if IS_DAY.is_some() == true {
                    IF_CONFLICT = true;
                }
                if IS_SEED.is_some() == true { 
                    IF_CONFLICT = true;
                }
            }
        }

        unsafe {
            if IF_ACCEPTABLE_SET.is_some() == true &&
                IF_FINAL_SET.is_some() == true {
                    if word_set_check() == false {
                        IF_CONFLICT = true;
                    }
                }
        }

        (word, random, difficult, stats)
    }

    pub fn if_number(s: String) -> Option<i32> {
        let mut ans: i32 = 0;
        let mut is_num: bool = true;

        for c in s.chars() {
                let this_num: i32 = 
                match c {
                    '0' => 0, '1' => 1, '2' => 2, '3' => 3, '4' => 4,
                    '5' => 5, '6' => 6, '7' => 7, '8' => 8, '9' => 9,
                    _ => -1
                };
                if this_num == -1 { is_num = false; break; }
                ans = ans * 10 + this_num;
            }
        
        if is_num == true {
            Some(ans)
        } else {
            None
        }
    }

    pub fn if_number_u64(s: String) -> Option<u64> {
        let mut ans: u64 = 0;
        let mut is_num: bool = true;

        for c in s.chars() {
                let this_num: u64 = 
                match c {
                    '0' => 0, '1' => 1, '2' => 2, '3' => 3, '4' => 4,
                    '5' => 5, '6' => 6, '7' => 7, '8' => 8, '9' => 9,
                    _ => 100
                };
                if this_num == 100 { is_num = false; break; }
                ans = ans * 10 + this_num;
            }
        
        if is_num == true {
            Some(ans)
        } else {
            None
        }
    }

    pub fn modify(mut x: i32) -> i32 {
        
        unsafe {
            if PLUS < 4 {
                PLUS += 1;
                x = x + 1;
            } else {
                PLUS = 0;
            }
        }
        x
    }

    pub fn word_set_check() -> bool {
        let mut b: bool = true;

        unsafe {
            for i in 0 ..= FINAL_SET.len() - 1 {
                let mut find: bool = false;
                for j in 0 ..= ACCEPTABLE_SET.len() - 1 {
                    if ACCEPTABLE_SET[j] == FINAL_SET[i] {
                        find = true;
                        break;
                    }
                }
                if find == false {
                    b = false;
                    break;
                }
            }
        }

        b
    }

}