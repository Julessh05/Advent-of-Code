pub mod second {
    use crate::second::second::second::Tendency::{Decrease, Increase};
    use std::fs;

    pub fn second_main() {
        second_main_one();
        second_main_two();
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
        // Stable isn't needed because this will automatically be a false case
    }

    fn second_main_one() {
        let list: Vec<String> = get_input();
        let mut safe_count: usize = list.len();
        for line in list {
            let levels: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>()).filter_map(Result::ok).collect();
            let mut tend: Option<Tendency> = None;
            for i in 0..levels.len() - 1 {
                if levels[i] < levels[i + 1] {
                    if tend.is_none() {
                        tend = Some(Increase);
                    } else if !matches!(tend, Some(Increase)) {
                        safe_count -= 1;
                        break;
                    }
                    if (1..=3).contains(&(levels[i] - levels[i + 1]).abs()) {
                        continue;
                    } else {
                        safe_count -= 1;
                        break;
                    }
                } else if levels[i] > levels[i + 1] {
                    if tend.is_none() {
                        tend = Some(Decrease);
                    } else if !matches!(tend, Some(Decrease)) {
                        safe_count -= 1;
                        break;
                    }
                    if (1..=3).contains(&(levels[i] - levels[i + 1]).abs()) {
                        continue;
                    } else {
                        safe_count -= 1;
                        break;
                    }
                } else {
                    safe_count -= 1;
                    break;
                }
            }
        }
        println!("Safe count: {}", safe_count);
    }

    fn second_main_two() {
        let list: Vec<String> = get_input();
        let mut safe_count: usize = list.len();
        for line in list {
            let mut levels: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>()).filter_map(Result::ok).collect();
            let mut tend: Option<Tendency> = None;
            let mut counter: usize = 0;
            let mut length: usize = levels.len() - 1;
            let mut did_remove: bool = false;
            while counter < length {
                if levels[counter] < levels[counter + 1] {
                    if tend.is_none() {
                        tend = Some(Increase);
                    } else if !matches!(tend, Some(Increase)) {
                        if !did_remove {
                            did_remove = true;
                            levels.remove(counter + 1);
                            length -= 1;
                            continue;
                        } else {
                            safe_count -= 1;
                            break;
                        }
                    }
                    if (1..=3).contains(&(levels[counter] - levels[counter + 1]).abs()) {
                        counter += 1;
                        continue;
                    } else {
                        if !did_remove {
                            did_remove = true;
                            levels.remove(counter + 1);
                            length -= 1;
                            continue;
                        } else {
                            safe_count -= 1;
                            break;
                        }
                    }
                } else if levels[counter] > levels[counter + 1] {
                    if tend.is_none() {
                        tend = Some(Decrease);
                    } else if !matches!(tend, Some(Decrease)) {
                        if !did_remove {
                            did_remove = true;
                            levels.remove(counter + 1);
                            length -= 1;
                            continue;
                        } else {
                            safe_count -= 1;
                            break;
                        }
                    }
                    if (1..=3).contains(&(levels[counter] - levels[counter + 1]).abs()) {
                        counter += 1;
                        continue;
                    } else {
                        if !did_remove {
                            did_remove = true;
                            levels.remove(counter + 1);
                            length -= 1;
                            continue;
                        } else {
                            safe_count -= 1;
                            break;
                        }
                    }
                } else {
                    if !did_remove {
                        did_remove = true;
                        levels.remove(counter + 1);
                        length -= 1;
                        continue;
                    } else {
                        safe_count -= 1;
                        break;
                    }
                }
            }
        }
        println!("Safe count with buffer: {}", safe_count);
    }
}
