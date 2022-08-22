pub mod tool {
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
                '!'
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

            let find = builtin_words::ACCEPTABLE.iter()
            .position(|&r| r == guess.as_str());
            if find.is_none() == true {
                return false
            }

        } else {
            return false
        }

        true
    }

    pub struct difficult_record {
        pub letter: i32,
        pub color: Color
    }

    pub fn difficult_valid(
        gus: &Vec<char>,
        dif_rec: &Vec<difficult_record>
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

        for arg in std::env::args() {

            if get_word == true {
                word = Some(arg);
                get_word = false;
                continue;
            }

            match arg.as_str() {
                "-w" | "--word" =>  get_word = true,
                "-r" | "--random" => random = true,
                "-D" | "--difficult" => difficult = true,
                "-t" | "--stats" => stats = true,
                _ => ()
            }
        }
        (word, random, difficult, stats)
    }


}