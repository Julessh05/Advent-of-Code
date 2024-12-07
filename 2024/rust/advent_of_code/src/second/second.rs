pub mod second {
    use crate::second::second::second::Tendency::{Decrease, Increase};
    use std::fs;

    pub fn second_main() {
        second_main_one();
        second_main_two();
    }

    static mut tend: Option<Tendency> = None;


    enum Tendency {
        Increase,
        Decrease,
        // Stable isn't needed because this will automatically be a false case
    }

    fn get_input() -> Vec<String> {
        let file_content = fs::read_to_string("data/test_input_second.txt").unwrap();
        let mut list: Vec<String> = vec![];
        for line in file_content.split("\n") {
            if line.is_empty() {
                break;
            }
            list.push(line.to_string());
        }
        list
    }

    /**
    This functions checks if the line is safe and
    returns the result as a boolean value
     */
    unsafe fn check_line(levels: Vec<i32>) -> bool {
        for i in 0..levels.len() - 1 {
            if levels[i] < levels[i + 1] {
                if tend.is_none() {
                    tend = Some(Increase);
                } else if !matches!(tend, Some(Increase)) {
                    return false;
                }
                if check_diff(levels[i], levels[i + 1]) {
                    continue;
                } else {
                    return false;
                }
            } else if levels[i] > levels[i + 1] {
                if tend.is_none() {
                    tend = Some(Decrease);
                } else if !matches!(tend, Some(Decrease)) {
                    return false;
                }
                if check_diff(levels[i], levels[i + 1]) {
                    continue;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    fn check_diff(first: i32, second: i32) -> bool {
        (1..=3).contains(&(first - second).abs())
    }

    fn second_main_one() {
        let list: Vec<String> = get_input();
        let mut safe_count: usize = list.len();
        for line in list {
            let levels: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>()).filter_map(Result::ok).collect();
            if !check_line(levels) {
                safe_count -= 1;
            }
        }
        println!("Safe count: {}", safe_count);
    }

    fn check_line_with_buffer(mut levels: Vec<i32>) -> bool {
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
                        return false;
                    }
                }
                if check_diff(levels[counter], levels[counter + 1]) {
                    counter += 1;
                    continue;
                } else {
                    if !did_remove {
                        did_remove = true;
                        levels.remove(counter + 1);
                        length -= 1;
                        continue;
                    } else {
                        return false;
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
                        return false;
                    }
                }
                if check_diff(levels[counter], levels[counter + 1]) {
                    counter += 1;
                    continue;
                } else {
                    if !did_remove {
                        did_remove = true;
                        levels.remove(counter + 1);
                        length -= 1;
                        continue;
                    } else {
                        return false;
                    }
                }
            } else {
                if !did_remove {
                    did_remove = true;
                    levels.remove(counter + 1);
                    length -= 1;
                    continue;
                } else {
                    return false;
                }
            }
        }
        true
    }

    fn check_line_with_buffer_counter(levels: Vec<i32>) -> bool {
        println!("{:?}", levels);
        let mut indice: Vec<usize> = vec![];
        let mut tend: Option<Tendency> = None;
        let tend_to_pass = tend;
        for i in 0..levels.len() - 1 {
            check_values(levels[i], levels[i + 1], tend_to_pass);
        }
        println!("{:?}", indice);
        if indice.len() < 2 {
            return true;
        } else if indice.len() < 3 {
            return indice[0] + 1 == indice[1];
        }
        false
    }

    fn check_values(one: i32, two: i32, tend: Option<Tendency>) -> (bool, Option<Tendency>) {
        let mut tend_to_return: Option<Tendency> = tend;
        if one < two {
            if tend_to_return.is_none() {
                tend_to_return = Some(Increase);
            } else if !matches!(tend_to_return, Some(Increase)) {
                return (false, tend_to_return);
            }
            (check_diff(one, two), tend_to_return)
        } else if one > two {
            if tend_to_return.is_none() {
                tend_to_return = Some(Decrease);
            } else if !matches!(tend_to_return, Some(Decrease)) {
                return (false, tend_to_return);
            }
            return (check_diff(one, two), tend_to_return);
        } else {
            return (false, tend_to_return);
        }
    }

    fn second_main_two() {
        let list: Vec<String> = get_input();
        let mut safe_count: usize = list.len();
        for line in list {
            let levels: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>()).filter_map(Result::ok).collect();
            if !check_line_with_buffer_counter(levels) {
                safe_count -= 1;
            }
        }
        println!("Safe count with buffer: {}", safe_count);
    }
}
