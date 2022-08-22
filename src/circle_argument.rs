pub mod game_circle {

    use crate::{interact_model::run_interact_model, test_model::run_test_model};
    use text_io::read;
    use crate::overall_situation::overall_variables;

    pub fn interact_model_circle() {
        let mut word: bool = false;
        let mut random: bool = false;
        let mut stats: bool = false;
        let mut suc_num: i32 = 0;
        let mut fail_num: i32 = 0;
        (word, random, stats) = run_interact_model::interact_run();

        if stats == true {
            unsafe {
                suc_num = overall_variables::success_num;
                fail_num = overall_variables::fail_num; 
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

        if word == false {
            loop {
                println!("Do you want to start another game? Y/N");
                let s: String = read!();
                if s == "Y" || s == "y" {
                    (word, random, stats) = run_interact_model::interact_run();
                } else {
                    break;
                }
                if stats == true {
                    unsafe {
                        suc_num = overall_variables::success_num;
                        fail_num = overall_variables::fail_num; 
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

    }

    pub fn test_model_circle() {
        let mut word: bool = false;
        let mut random: bool = false;
        let mut stats: bool = false;
        let mut suc_num: i32 = 0;
        let mut fail_num: i32 = 0;
        (word, random, stats) = run_test_model::test_run();

        if stats == true {
            unsafe {
                suc_num = overall_variables::success_num;
                fail_num = overall_variables::fail_num; 
            }
            println!("{} {} {:.2}", suc_num, fail_num,
                overall_variables::try_times_on_average());
            overall_variables::print_frequent_test();
        }

        if word == false {
            loop {
                let s: String = read!();
                if s == "Y" {
                    //println!("circle!!!");
                    (word, random, stats) = run_test_model::test_run();
                } else {
                    break;
                }
                if stats == true {
                    unsafe {
                        suc_num = overall_variables::success_num;
                        fail_num = overall_variables::fail_num; 
                    }
                    println!("{} {} {:.2}", suc_num, fail_num,
                        overall_variables::try_times_on_average());
                    overall_variables::print_frequent_test();
                }
            }
        }

    }

}