pub mod process_json {

    use serde_derive::{ Serialize, Deserialize };
    use crate::overall_situation::overall_variables;

    pub static mut before_win: i32 = 0;
    pub static mut before_lose: i32 = 0;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Games {
        pub answer: String,
        pub guesses: Vec<String>
    }

    #[derive(Deserialize, Serialize, Debug, )]
    pub struct State {
        pub total_rounds: i32,
        pub games: Vec<Games>
    }

    pub fn before_load_json(address: String) {

        let mut json_record = {
            let json_record = std::fs::read_to_string(&address);
    
            // Load the State structure from the string.
            let s: String = json_record.unwrap();
            if s.clone() == "{}" {
                State {
                    total_rounds: 0,
                    games: Vec::new()
                }
            } else {
                serde_json::from_str::<State>(&s).unwrap()
            }
        };

        unsafe {
            for i in 0 ..= json_record.total_rounds - 1 {
                let mut game: &Games = &json_record.games[i as usize];
                let mut l: usize = game.guesses.len();
                if game.answer == game.guesses[l - 1] {
                    before_win += 1;
                    overall_variables::try_times.push(l as i32);
                } else {
                    before_lose += 1;
                }
                for j in 0 ..= l - 1 {
                    overall_variables::record_use_times(game.guesses[j].clone().to_lowercase());
                }
            }
            overall_variables::success_num += before_win;
            overall_variables::fail_num += before_lose;
        }
    }

    pub fn load_json(address: String) {

        let mut json_record = {
            let json_record = std::fs::read_to_string(&address);
    
            // Load the State structure from the string.
            let s: String = json_record.unwrap();
            //println!("{:#?}", s);
            if s.clone() == "{}" {
                State {
                    total_rounds: 0,
                    games: Vec::new()
                }
            } else {
                serde_json::from_str::<State>(&s).unwrap()
                /*State {
                    total_rounds: 0,
                    all_games: Vec::new()
                }*/
            }
        };
 
        unsafe {
            json_record.total_rounds = overall_variables::success_num
                + overall_variables::fail_num;
            for i in 0 ..= overall_variables::GamesRecord.len() - 1 {
                json_record.games.push(
                    Games {
                        answer: overall_variables::GamesRecord[i].answer.clone(),
                        guesses: overall_variables::GamesRecord[i].guesses.clone() 
                    }
                )
            } // modify the json file
        }

        std::fs::write(
            &address,
            serde_json::to_string_pretty(&json_record).unwrap(),
        ); // write in the json file

    }

    pub fn test_load_json(address: String) {

        let json_record = std::fs::read_to_string(&address);
    
        // Load the State structure from the string.
        let s: String = json_record.unwrap();
        if s.clone() != "{}" {
            if serde_json::from_str::<State>(&s).is_err() == true {
                panic!("Wrong JSON!!!")
            }
        }
    
    }

}