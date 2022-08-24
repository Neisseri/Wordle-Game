pub mod game_circle {

    use crate::{interact_model::run_interact_model, 
        test_model::run_test_model, 
        overall_situation::overall_variables::IF_STATE, 
        parse_json};
    use text_io::read;
    use crate::overall_situation::overall_variables;

    pub fn interact_model_circle() {
        let mut _word: bool;
        let mut _random: bool;
        let mut stats: bool;
        let mut suc_num: i32;
        let mut fail_num: i32;
        (_word, _random, stats) = run_interact_model::interact_run();

        // json
        unsafe {
            if IF_STATE == true {
                parse_json::process_json::before_load_json(
                    overall_variables::JSON_ADDRESS.clone());
            }
        }

        if stats == true {
            unsafe {
                suc_num = overall_variables::SUCCESS_NUM;
                fail_num = overall_variables::FAIL_NUM; 
            }
            println!("You winned {} games, lost {} games!",
                console::style(suc_num).bold().blink().blue(),
                console::style(fail_num).bold().blink().blue()
            );
            println!("Your chance of winning is: {:.2}",
                console::style(overall_variables::win_chance())
                .bold().blink().blue()
            );
            println!("You try {:.2} times on average!",
                console::style(overall_variables::try_times_on_average())
                .bold().blink().blue()
            );
            println!("{}", 
                console::style("The words that you use most frequently are:")
                .blink().yellow());
            overall_variables::print_frequent();
        }

        if _word == false {
            loop {
                println!("{}",
                    console::style("Do you want to start another game? Y/N")
                    .blink().on_red());
                let s: String = read!();
                if s == "Y" || s == "y" {
                    (_word, _random, stats) = run_interact_model::interact_run();
                } else {
                    break;
                }
                if stats == true {
                    unsafe {
                        suc_num = overall_variables::SUCCESS_NUM;
                        fail_num = overall_variables::FAIL_NUM; 
                    }
                    println!("You winned {} games, lost {} games!",
                        console::style(suc_num).bold().blink().blue(),
                        console::style(fail_num).bold().blink().blue()
                    );
                    println!("Your chance of winning is: {:.2}",
                        console::style(overall_variables::win_chance())
                        .bold().blink().blue()
                    );
                    println!("You try {:.2} times on average!",
                        console::style(overall_variables::try_times_on_average())
                        .bold().blink().blue()
                    );
                    println!("The words that you use most frequently are:");
                    overall_variables::print_frequent();
                }
            }
        }

        unsafe {
            if IF_STATE == true {
                parse_json::process_json::load_json(
                    overall_variables::JSON_ADDRESS.clone());
            }
        }

    }

    pub fn test_model_circle() {
        let mut _word: bool;
        let mut _random: bool;
        let mut stats: bool;
        let mut suc_num: i32;
        let mut fail_num: i32;

        (_word, _random, stats) = run_test_model::test_run();

        // json
        unsafe {
            if IF_STATE == true {
                parse_json::process_json::before_load_json(
                    overall_variables::JSON_ADDRESS.clone());
            }
        }

        if stats == true {
            unsafe {
                suc_num = overall_variables::SUCCESS_NUM;
                fail_num = overall_variables::FAIL_NUM;
            }
            println!("{} {} {:.2}", suc_num, fail_num,
                overall_variables::try_times_on_average());
            overall_variables::print_frequent_test();
        }

        if _word == false {
            loop {
                let s: String = read!();
                if s == "Y" {
                    //println!("circle!!!");
                    (_word, _random, stats) = run_test_model::test_run();
                } else {
                    break;
                }
                if stats == true {
                    unsafe {
                        suc_num = overall_variables::SUCCESS_NUM;
                        fail_num = overall_variables::FAIL_NUM; 
                    }
                    println!("{} {} {:.2}", suc_num, fail_num,
                        overall_variables::try_times_on_average());
                    overall_variables::print_frequent_test();
                }
            }
        }

        unsafe {
            if IF_STATE == true {
                parse_json::process_json::load_json(
                    overall_variables::JSON_ADDRESS.clone());
            }
        }

    }

}