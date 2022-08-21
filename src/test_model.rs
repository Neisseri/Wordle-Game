use text_io::read;

pub mod tool {
    pub fn match_words(word: char) -> usize {
        match word {
            'a' => 0, 'b' => 1, 'c' => 2, 'd' => 3, 'e' => 4,
            'f' => 5, 'g' => 6, 'h' => 7, 'i' => 8, 'j' => 9,
            'k' => 10, 'l' => 11, 'm' => 12, 'n' => 13, 'o' => 14,
            'p' => 15, 'q' => 16, 'r' => 17, 's' => 18, 't' => 19,
            'u' => 20, 'v' => 21, 'w' => 22, 'x' => 23, 'y' => 24,
            'z' => 25,
            _ => panic!("No this word!")
        }
    }

    #[derive(Clone)]
    pub enum Color {
        Red,
        Yellow,
        Green,
        Unknown
    }
}

pub mod run_test_model {

    use super::tool::{match_words, Color};

    pub fn test_run() -> () {
        let answer: String = text_io::read!();
        let mut ans: Vec<char> = Vec::new();
        for words in answer.chars() {
            ans.push(words);
        } // read the Final words and convert into vector

        let mut count: Vec<i32> = vec![0; 26];
        for i in 0 ..= 4 {
            count[match_words(ans[i])] += 1;
        }

        let mut guess: String;
        let mut alphabet: Vec<char> = Vec::new();
        for _ in 1 ..= 6 {
            guess = text_io::read!();
            let mut gus: Vec<char> = Vec::new();
            for words in guess.chars() {
                gus.push(words);
            } // read the input words and convert into chars

            let mut colors: Vec<Color> = Vec::new();//the color of XXXXX

            let mut cnt_gus: Vec<i32> = vec![0; 26];
            // the number of letters in the guessed word

            let mut kryboard: Vec<Color> = vec![Color::Unknown; 26];

            for i in 0 ..= 4 {
                cnt_gus[match_words(gus[i])] += 1;
                //count the number of used letters

                if gus[i] == ans[i] {
                    colors[i] = Color::Green;
                } else {
                    if cnt_gus[match_words(gus[i])] <= count[match_words(gus[i])]
                    {
                        colors[i] = Color::Yellow;
                    } else {
                        colors[i] = Color::Red;
                    }
                } // give the color of XXXXX


            } 


        }

    }
}