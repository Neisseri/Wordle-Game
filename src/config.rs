pub mod Configuration {

    use serde_derive::{ Serialize, Deserialize };
    use crate::{overall_situation::overall_variables, my_tool::tool::difficult_record};

    #[derive(Serialize, Deserialize)]
    pub struct CFG {
        random: Option<bool>, // 0
        difficult: Option<bool>, // 1
        stats: Option<bool>, // 2
        day: Option<i32>, // 3
        seed: Option<u64>, // 4
        final_set: Option<String>, // 5
        acceptable_set: Option<String>, // 6
        state: Option<String>, // 7
        word: Option<String> // 8
    }

    pub fn parse_config(address: String) -> (Option<String>, bool, bool, bool) {

        let mut word: Option<String> = None;
        let mut random: bool = false;
        let mut difficult: bool = false;
        let mut stats: bool = false;

        let mut json_record = {
            let json_record = std::fs::read_to_string(&address);
    
            // Load the CFG structure from the string.
            let s: String = json_record.unwrap();
            serde_json::from_str::<CFG>(&s).unwrap()
        };

        unsafe {
            if json_record.day.is_some() == true 
                && overall_variables::config_def[3] == false {
                overall_variables::is_day = json_record.day.clone();
            }
            if json_record.seed.is_some() == true 
                && overall_variables::config_def[4] == false {
                overall_variables::is_seed = json_record.seed.clone();
            }
            if json_record.final_set.is_some() == true
                && overall_variables::config_def[5] == false {
                overall_variables::if_final_set = json_record.final_set.clone();
                overall_variables::read_final_set(
                    overall_variables::if_final_set.clone().unwrap());
            }
            if json_record.acceptable_set.is_some() == true 
                && overall_variables::config_def[6] == false {
                overall_variables::if_acceptable_set = json_record.acceptable_set.clone();
                overall_variables::read_acceptable_set(
                    overall_variables::if_acceptable_set.clone().unwrap());
            }
            if json_record.state.is_some() == true 
                && overall_variables::config_def[7] == false {
                overall_variables::if_state = true;
                overall_variables::json_address = json_record.state.unwrap();
            }
            if json_record.word.is_some() == true 
                && overall_variables::config_def[8] == false {
                word = json_record.word.clone();
            }
            if json_record.random.is_some() == true
                && overall_variables::config_def[0] == false {
                random = json_record.random.unwrap();
            }
            if json_record.difficult.is_some() == true 
                && overall_variables::config_def[1] == false {
                difficult = json_record.difficult.unwrap();
            }
            if json_record.stats.is_some() == true 
                && overall_variables::config_def[2] == false {
                stats = json_record.stats.unwrap();
            }
        }
        (word, random, difficult, stats)
    }
}