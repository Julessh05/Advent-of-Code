pub mod second {
    use crate::second::second::second::Tendency::{Decrease, Increase, Stable};
    use std::fs;

    pub fn second_main() {
        second_main_one();
    }

    fn get_input() -> Vec<String> {
        let file_content = fs::read_to_string("data/input_second.txt").unwrap();
        let mut list: Vec<String> = vec![];
        for line in file_content.split("\n") {
            if line.is_empty() {
                break;
            }
            list.push(line.to_string());
        }
        list
    }

    enum Tendency {
        Increase,
        Decrease,
        Stable,
    }

    fn second_main_one() {
        let list: Vec<String> = get_input();
        let mut safe_count: usize = list.len();
        for line in list {
            let levels: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>()).filter_map(Result::ok).collect();
            let mut is_safe: bool = true;
            let mut tend: Option<Tendency> = None;
            for i in 0..levels.len() - 1 {
                if !is_safe {
                    safe_count -= 1;
                    break;
                }
                if levels[i] > levels[i + 1] {
                    if tend.is_none() {
                        tend = Option::from(Increase);
                    } else if !matches!(tend, Some(Increase)) {
                        is_safe = false;
                        break;
                    }
                    if (1..=3).contains(&(levels[i] - levels[i + 1].abs())) {
                        continue;
                    } else {
                        is_safe = false;
                        break;
                    }
                } else if levels[i] < levels[i + 1] {
                    if tend.is_none() {
                        tend = Option::from(Decrease);
                    } else if !matches!(tend, Some(Decrease)) {
                        is_safe = false;
                        break;
                    }
                    if (1..=3).contains(&(levels[i] - levels[i + 1].abs())) {
                        continue;
                    } else {
                        is_safe = false;
                        break;
                    }
                } else {
                    tend = Some(Stable);
                    is_safe = false;
                    break;
                }
            }
        }
        println!("Safe count: {}", safe_count);
    }

    pub fn second_main_two() {}
}
